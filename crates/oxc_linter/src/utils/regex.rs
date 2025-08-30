use oxc_allocator::Allocator;
use oxc_ast::{
    AstKind,
    ast::{Argument, Expression},
};
use oxc_regular_expression::{ConstructorParser, Options, ast::Pattern};
use oxc_semantic::IsGlobalReference;
use oxc_span::Span;

use crate::{AstNode, context::LintContext};

pub fn run_on_regex_node<'a, 'b, M>(node: &'a AstNode<'b>, ctx: &'a LintContext<'b>, cb: M)
where
    M: FnOnce(&Pattern<'_>, Span),
{
    match node.kind() {
        AstKind::RegExpLiteral(reg) => {
            if let Some(pat) = &reg.regex.pattern.pattern {
                cb(pat, reg.span);
            }
        }
        AstKind::NewExpression(expr) if is_regexp_callee(&expr.callee, ctx) => {
            // note: improvements required for strings used via identifier references
            // Missing or non-string arguments will be runtime errors, but are not covered by this rule.
            match (&expr.arguments.first(), &expr.arguments.get(1)) {
                (Some(Argument::StringLiteral(pattern)), Some(Argument::StringLiteral(flags))) => {
                    let allocator = Allocator::default();
                    if let Some(pat) = parse_regex(&allocator, pattern.span, Some(flags.span), ctx)
                    {
                        cb(&pat, pattern.span);
                    }
                }
                (Some(Argument::StringLiteral(pattern)), _) => {
                    let allocator = Allocator::default();
                    if let Some(pat) = parse_regex(&allocator, pattern.span, None, ctx) {
                        cb(&pat, pattern.span);
                    }
                }
                _ => {}
            }
        }

        // RegExp()
        AstKind::CallExpression(expr) if is_regexp_callee(&expr.callee, ctx) => {
            // note: improvements required for strings used via identifier references
            // Missing or non-string arguments will be runtime errors, but are not covered by this rule.
            match (&expr.arguments.first(), &expr.arguments.get(1)) {
                (Some(Argument::StringLiteral(pattern)), Some(Argument::StringLiteral(flags))) => {
                    let allocator = Allocator::default();
                    if let Some(pat) = parse_regex(&allocator, pattern.span, Some(flags.span), ctx)
                    {
                        cb(&pat, pattern.span);
                    }
                }
                (Some(Argument::StringLiteral(pattern)), _) => {
                    let allocator = Allocator::default();
                    if let Some(pat) = parse_regex(&allocator, pattern.span, None, ctx) {
                        cb(&pat, pattern.span);
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

// Accepts both RegExp and globalThis.RegExp
fn is_regexp_callee<'a>(callee: &'a Expression<'a>, ctx: &'a LintContext<'_>) -> bool {
    if callee.is_global_reference_name("RegExp", ctx.semantic().scoping()) {
        return true;
    }
    // Check for globalThis.RegExp (StaticMemberExpression)
    if let Expression::StaticMemberExpression(member) = callee {
        if let Expression::Identifier(obj) = &member.object {
            if obj.is_global_reference_name("globalThis", ctx.semantic().scoping())
                && member.property.name == "RegExp"
            {
                return true;
            }
        }
    }
    false
}

fn parse_regex<'a>(
    allocator: &'a Allocator,
    pattern_span: Span,
    flags_span: Option<Span>,
    ctx: &'a LintContext<'_>,
) -> Option<Pattern<'a>> {
    let flags_text = flags_span.map(|span| span.source_text(ctx.source_text()));
    let parser = ConstructorParser::new(
        allocator,
        pattern_span.source_text(ctx.source_text()),
        flags_text,
        Options {
            pattern_span_offset: pattern_span.start,
            flags_span_offset: flags_span.map_or(0, |span| span.start),
        },
    );
    let Ok(pattern) = parser.parse() else { return None };
    Some(pattern)
}
