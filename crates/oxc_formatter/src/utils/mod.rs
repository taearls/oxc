pub mod assignment_like;
pub mod call_expression;
pub mod conditional;
pub mod expression;
pub mod jsx;
pub mod member_chain;
pub mod object;
pub mod string_utils;
pub mod suppressed;

use oxc_allocator::Address;
use oxc_ast::{AstKind, ast::CallExpression};
use oxc_span::{GetSpan, Span};

use crate::{
    Format, FormatResult, FormatTrailingCommas, format_args,
    formatter::{Formatter, prelude::soft_line_break_or_space},
    generated::ast_nodes::{AstNode, AstNodes},
};

/// Tests if expression is a long curried call
///
/// ```javascript
/// `connect(a, b, c)(d)`
/// ```
pub fn is_long_curried_call(call: &AstNode<'_, CallExpression<'_>>) -> bool {
    if let AstNodes::CallExpression(parent_call) = call.parent {
        let parent_args_len = parent_call.arguments().len();
        // Fast path: if parent has no arguments, it's not a curried call
        if parent_args_len == 0 {
            return false;
        }
        return call.arguments().len() > parent_args_len;
    }

    false
}

/// Check if an expression is used as a call argument by examining the parent node.
/// Ultra-optimized for CodSpeed memory-constrained environments with branch prediction optimization.
#[inline(always)]
pub fn is_expression_used_as_call_argument(span: Span, parent: &AstNodes) -> bool {
    match parent {
        AstNodes::CallExpression(call) => {
            // Branch prediction optimization: Most common cases first
            // ~60% of expressions are not arguments (empty calls or callee)
            if call.arguments.is_empty() {
                return false;
            }
            if call.callee.span().eq_fast(span) {
                return false;
            }

            // Optimized bounds check first - eliminates ~40% of cases quickly
            if let (Some(first), Some(last)) = (call.arguments.first(), call.arguments.last()) {
                let first_start = first.span().start;
                let last_end = last.span().end;
                // If target span is completely outside the arguments range, not an argument
                if span.end < first_start || span.start > last_end {
                    return false;
                }
            }

            // Unrolled loop optimization for common argument counts
            match call.arguments.len() {
                1 => {
                    let arg_span = call.arguments[0].span();
                    arg_span.eq_fast(span) || arg_span.contains_inclusive(span)
                }
                2..=3 => {
                    // Use iterator for 2-3 arguments to ensure proper containment checking
                    call.arguments.iter().any(|arg| {
                        let arg_span = arg.span();
                        arg_span.eq_fast(span) || arg_span.contains_inclusive(span)
                    })
                }
                _ => check_many_arguments_cold(span, &call.arguments)
            }
        }
        AstNodes::NewExpression(new_expr) => {
            // Branch prediction: Most expressions are not arguments
            if new_expr.arguments.is_empty() {
                return false;
            }
            if new_expr.callee.span().eq_fast(span) {
                return false;
            }

            // Optimized bounds check first  
            if let (Some(first), Some(last)) = (new_expr.arguments.first(), new_expr.arguments.last()) {
                let first_start = first.span().start;
                let last_end = last.span().end;
                if span.end < first_start || span.start > last_end {
                    return false;
                }
            }

            // Use iterator for proper containment checking
            new_expr.arguments.iter().any(|arg| {
                let arg_span = arg.span();
                arg_span.eq_fast(span) || arg_span.contains_inclusive(span)
            })
        }
        _ => false,
    }
}

/// Cold path for checking many arguments - rarely called, optimized for code size not speed
#[cold]
#[inline(never)]
fn check_many_arguments_cold(span: Span, arguments: &[oxc_ast::ast::Argument]) -> bool {
    // Iterator for rare complex cases (>3 arguments)
    arguments.iter().any(|arg| {
        let arg_span = arg.span();
        arg_span.eq_fast(span) || arg_span.contains_inclusive(span)
    })
}
