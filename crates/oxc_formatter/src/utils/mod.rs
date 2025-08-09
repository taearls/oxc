pub mod assignment_like;
pub mod call_expression;
pub mod conditional;
pub mod member_chain;

use oxc_allocator::Address;
use oxc_ast::{AstKind, ast::CallExpression};
use oxc_span::GetSpan;

use crate::{
    Format, FormatResult, FormatTrailingCommas, format_args,
    formatter::{Formatter, prelude::soft_line_break_or_space},
    generated::ast_nodes::{AstNode, AstNodes},
};

/// This function is in charge to format the call arguments.
pub fn write_arguments_multi_line<'a, S: Format<'a>, I>(
    separated: I,
    f: &mut Formatter<'_, 'a>,
) -> FormatResult<()>
where
    I: Iterator<Item = S>,
{
    let mut iterator = separated.peekable();
    let mut join_with = f.join_with(soft_line_break_or_space());

    while let Some(element) = iterator.next() {
        let last = iterator.peek().is_none();

        if last {
            join_with.entry(&format_args!(element, FormatTrailingCommas::All));
        } else {
            join_with.entry(&element);
        }
    }

    join_with.finish()
}

/// Tests if expression is a long curried call
///
/// ```javascript
/// `connect(a, b, c)(d)`
/// ```
pub fn is_long_curried_call(call: &AstNode<'_, CallExpression<'_>>) -> bool {
    if let AstNodes::CallExpression(parent_call) = call.parent {
        // Check if this is a true curried call pattern: connect(a, b, c)(d)
        // This means the parent call's callee should be this call expression.
        // If this call is just an argument to the parent, it's not a curried call.

        // Get the span of this call expression
        let this_call_span = call.span();

        // Check if this call is the callee of the parent call (true curried pattern)
        let is_true_curried = parent_call.callee.span() == this_call_span;

        if is_true_curried {
            return call.arguments().len() > parent_call.arguments().len()
                && !parent_call.arguments().is_empty();
        } else {
            return false;
        }
    }

    false
}
