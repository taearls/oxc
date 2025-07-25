use std::cell::Cell;

use oxc_allocator::{Box as ArenaBox, CloneIn, Vec as ArenaVec};
use oxc_ast::ast::*;
use oxc_ast_visit::{Visit, VisitMut, walk_mut::walk_ts_signatures};
use oxc_ecmascript::BoundNames;
use oxc_span::{GetSpan, SPAN};
use oxc_syntax::scope::ScopeFlags;

use crate::{
    IsolatedDeclarations,
    diagnostics::{
        accessor_must_have_explicit_return_type, binding_element_export,
        inferred_type_of_expression, signature_computed_property_name,
        variable_must_have_explicit_type,
    },
};

impl<'a> IsolatedDeclarations<'a> {
    pub(crate) fn transform_variable_declaration(
        &self,
        decl: &VariableDeclaration<'a>,
        check_binding: bool,
    ) -> Option<ArenaBox<'a, VariableDeclaration<'a>>> {
        if decl.declare {
            None
        } else {
            let declarations =
                self.ast.vec_from_iter(decl.declarations.iter().filter_map(|declarator| {
                    self.transform_variable_declarator(declarator, check_binding)
                }));
            Some(self.transform_variable_declaration_with_new_declarations(decl, declarations))
        }
    }

    pub(crate) fn transform_variable_declaration_with_new_declarations(
        &self,
        decl: &VariableDeclaration<'a>,
        declarations: ArenaVec<'a, VariableDeclarator<'a>>,
    ) -> ArenaBox<'a, VariableDeclaration<'a>> {
        self.ast.alloc_variable_declaration(decl.span, decl.kind, declarations, self.is_declare())
    }

    pub(crate) fn transform_variable_declarator(
        &self,
        decl: &VariableDeclarator<'a>,
        check_binding: bool,
    ) -> Option<VariableDeclarator<'a>> {
        if decl.id.kind.is_destructuring_pattern() {
            decl.id.bound_names(&mut |id| {
                if !check_binding || self.scope.has_value_reference(&id.name) {
                    self.error(binding_element_export(id.span));
                }
            });
            return None;
        }

        if check_binding {
            if let Some(name) = decl.id.get_identifier_name() {
                if !self.scope.has_value_reference(&name) {
                    return None;
                }
            }
        }

        let mut binding_type = None;
        let mut init = None;
        if decl.id.type_annotation.is_none() {
            if let Some(init_expr) = &decl.init {
                // if kind is const and it doesn't need to infer type from expression
                if decl.kind.is_const() && !Self::is_need_to_infer_type_from_expression(init_expr) {
                    if let Expression::TemplateLiteral(lit) = init_expr {
                        init =
                            self.transform_template_to_string(lit).map(Expression::StringLiteral);
                    } else {
                        init = Some(init_expr.clone_in(self.ast.allocator));
                    }
                } else if !decl.kind.is_const()
                    || !matches!(init_expr, Expression::TemplateLiteral(_))
                {
                    // otherwise, we need to infer type from expression
                    binding_type = self.infer_type_from_expression(init_expr);
                }
            }
            if init.is_none() && binding_type.is_none() {
                binding_type = Some(self.ast.ts_type_unknown_keyword(SPAN));
                if !decl.init.as_ref().is_some_and(Expression::is_function) {
                    self.error(variable_must_have_explicit_type(decl.id.span()));
                }
            }
        }
        let id = binding_type.map_or_else(
            || decl.id.clone_in(self.ast.allocator),
            |ts_type| {
                self.ast.binding_pattern(
                    decl.id.kind.clone_in(self.ast.allocator),
                    Some(self.ast.ts_type_annotation(SPAN, ts_type)),
                    decl.id.optional,
                )
            },
        );

        Some(self.ast.variable_declarator(decl.span, decl.kind, id, init, decl.definite))
    }

    fn transform_ts_module_block(
        &mut self,
        block: &ArenaBox<'a, TSModuleBlock<'a>>,
    ) -> ArenaBox<'a, TSModuleBlock<'a>> {
        // We need to enter a new scope for the module block, avoid add binding to the parent scope
        // TODO: doesn't have a scope_id!
        self.scope.enter_scope(ScopeFlags::TsModuleBlock, &Cell::default());
        let stmts = self.transform_statements_on_demand(&block.body);
        self.scope.leave_scope();
        self.ast.alloc_ts_module_block(SPAN, self.ast.vec(), stmts)
    }

    pub(crate) fn transform_ts_module_declaration(
        &mut self,
        decl: &ArenaBox<'a, TSModuleDeclaration<'a>>,
    ) -> ArenaBox<'a, TSModuleDeclaration<'a>> {
        if decl.declare {
            return decl.clone_in(self.ast.allocator);
        }

        let Some(body) = &decl.body else {
            return decl.clone_in(self.ast.allocator);
        };

        match body {
            TSModuleDeclarationBody::TSModuleDeclaration(decl) => {
                let inner = self.transform_ts_module_declaration(decl);
                self.ast.alloc_ts_module_declaration(
                    decl.span,
                    decl.id.clone_in(self.ast.allocator),
                    Some(TSModuleDeclarationBody::TSModuleDeclaration(inner)),
                    decl.kind,
                    self.is_declare(),
                )
            }
            TSModuleDeclarationBody::TSModuleBlock(block) => {
                let body = self.transform_ts_module_block(block);
                self.ast.alloc_ts_module_declaration(
                    decl.span,
                    decl.id.clone_in(self.ast.allocator),
                    Some(TSModuleDeclarationBody::TSModuleBlock(body)),
                    decl.kind,
                    self.is_declare(),
                )
            }
        }
    }

    pub(crate) fn transform_declaration(
        &mut self,
        decl: &Declaration<'a>,
        check_binding: bool,
    ) -> Option<Declaration<'a>> {
        match decl {
            Declaration::FunctionDeclaration(func) => {
                let needs_transform = !check_binding
                    || func.id.as_ref().is_some_and(|id| self.scope.has_value_reference(&id.name));
                needs_transform
                    .then(|| Declaration::FunctionDeclaration(self.transform_function(func, None)))
            }
            Declaration::VariableDeclaration(decl) => self
                .transform_variable_declaration(decl, check_binding)
                .map(Declaration::VariableDeclaration),
            Declaration::ClassDeclaration(decl) => {
                let needs_transform = !check_binding
                    || decl.id.as_ref().is_some_and(|id| self.scope.has_reference(&id.name));
                needs_transform
                    .then(|| Declaration::ClassDeclaration(self.transform_class(decl, None)))
            }
            Declaration::TSTypeAliasDeclaration(alias_decl) => {
                if !check_binding || self.scope.has_reference(&alias_decl.id.name) {
                    let mut decl = decl.clone_in(self.ast.allocator);
                    self.visit_declaration(&mut decl);
                    Some(decl)
                } else {
                    None
                }
            }
            Declaration::TSInterfaceDeclaration(interface_decl) => {
                if !check_binding || self.scope.has_reference(&interface_decl.id.name) {
                    let mut decl = decl.clone_in(self.ast.allocator);
                    self.visit_declaration(&mut decl);
                    Some(decl)
                } else {
                    None
                }
            }
            Declaration::TSEnumDeclaration(enum_decl) => {
                if !check_binding || self.scope.has_reference(&enum_decl.id.name) {
                    Some(self.transform_ts_enum_declaration(enum_decl))
                } else {
                    None
                }
            }
            Declaration::TSModuleDeclaration(decl) => {
                if !check_binding
                    || matches!(
                        &decl.id,
                        TSModuleDeclarationName::Identifier(ident)
                            if self.scope.has_value_reference(&ident.name)
                    )
                {
                    Some(Declaration::TSModuleDeclaration(
                        self.transform_ts_module_declaration(decl),
                    ))
                } else {
                    None
                }
            }
            Declaration::TSImportEqualsDeclaration(decl) => {
                if !check_binding || self.scope.has_reference(&decl.id.name) {
                    Some(Declaration::TSImportEqualsDeclaration(decl.clone_in(self.ast.allocator)))
                } else {
                    None
                }
            }
        }
    }

    fn report_signature_property_key(&self, key: &PropertyKey<'a>, computed: bool) {
        if !computed {
            return;
        }

        let is_not_allowed = match key {
            PropertyKey::StaticIdentifier(_) | PropertyKey::Identifier(_) => false,
            PropertyKey::StaticMemberExpression(expr) => {
                !expr.get_first_object().is_identifier_reference()
            }
            key => !Self::is_literal_key(key),
        };

        if is_not_allowed {
            self.error(signature_computed_property_name(key.span()));
        }
    }
}

impl<'a> VisitMut<'a> for IsolatedDeclarations<'a> {
    fn visit_ts_signatures(&mut self, signatures: &mut ArenaVec<'a, TSSignature<'a>>) {
        self.transform_ts_signatures(signatures);
        walk_ts_signatures(self, signatures);
    }

    fn visit_ts_method_signature(&mut self, signature: &mut TSMethodSignature<'a>) {
        self.report_signature_property_key(&signature.key, signature.computed);
        if signature.return_type.is_none() {
            match signature.kind {
                TSMethodSignatureKind::Method => {
                    self.error(inferred_type_of_expression(signature.span));
                }
                TSMethodSignatureKind::Get => {
                    self.error(accessor_must_have_explicit_return_type(signature.key.span()));
                }
                TSMethodSignatureKind::Set => {
                    // setter method don't need return type
                }
            }
        }
    }

    fn visit_ts_property_signature(&mut self, signature: &mut TSPropertySignature<'a>) {
        self.report_signature_property_key(&signature.key, signature.computed);
    }
}
