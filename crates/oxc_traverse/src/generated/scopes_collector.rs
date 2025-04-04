// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/generators/scopes_collector.rs`

use std::cell::Cell;

use oxc_ast::ast::*;
use oxc_ast_visit::Visit;
use oxc_syntax::scope::{ScopeFlags, ScopeId};

/// Visitor that locates all child scopes.
/// NB: Child scopes only, not grandchild scopes.
/// Does not do full traversal - stops each time it hits a node with a scope.
pub struct ChildScopeCollector {
    pub(crate) scope_ids: Vec<ScopeId>,
}

impl ChildScopeCollector {
    pub(crate) fn new() -> Self {
        Self { scope_ids: vec![] }
    }

    pub(crate) fn add_scope(&mut self, scope_id: &Cell<Option<ScopeId>>) {
        self.scope_ids.push(scope_id.get().unwrap());
    }
}

impl<'a> Visit<'a> for ChildScopeCollector {
    #[inline]
    fn visit_program(&mut self, it: &Program<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_block_statement(&mut self, it: &BlockStatement<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_for_statement(&mut self, it: &ForStatement<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_for_in_statement(&mut self, it: &ForInStatement<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_for_of_statement(&mut self, it: &ForOfStatement<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_switch_statement(&mut self, it: &SwitchStatement<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_catch_clause(&mut self, it: &CatchClause<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_function(&mut self, it: &Function<'a>, _: ScopeFlags) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_arrow_function_expression(&mut self, it: &ArrowFunctionExpression<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_class(&mut self, it: &Class<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_static_block(&mut self, it: &StaticBlock<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_ts_enum_declaration(&mut self, it: &TSEnumDeclaration<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_ts_conditional_type(&mut self, it: &TSConditionalType<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_ts_type_alias_declaration(&mut self, it: &TSTypeAliasDeclaration<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_ts_interface_declaration(&mut self, it: &TSInterfaceDeclaration<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_ts_method_signature(&mut self, it: &TSMethodSignature<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_ts_construct_signature_declaration(
        &mut self,
        it: &TSConstructSignatureDeclaration<'a>,
    ) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_ts_module_declaration(&mut self, it: &TSModuleDeclaration<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_ts_function_type(&mut self, it: &TSFunctionType<'a>) {
        self.add_scope(&it.scope_id);
    }

    #[inline]
    fn visit_ts_mapped_type(&mut self, it: &TSMappedType<'a>) {
        self.add_scope(&it.scope_id);
    }
}
