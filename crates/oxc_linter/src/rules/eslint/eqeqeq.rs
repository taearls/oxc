use oxc_ast::{
    ast::{BinaryExpression, Expression},
    AstKind,
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::{GetSpan, Span};
use oxc_syntax::operator::{BinaryOperator, UnaryOperator};

use crate::{context::LintContext, rule::Rule, AstNode};

fn eqeqeq_diagnostic(x0: &str, x1: &str, span2: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn(format!("Expected {x1} and instead saw {x0}"))
        .with_help(format!("Prefer {x1} operator"))
        .with_label(span2)
}

#[derive(Debug, Default, Clone)]
pub struct Eqeqeq {
    compare_type: CompareType,
    null_type: NullType,
}

declare_oxc_lint!(
    /// ### What it does
    /// Requires the use of the === and !== operators
    ///
    /// ### Why is this bad?
    /// Using non-strict equality operators leads to hard to track bugs due to type coercion.
    ///
    /// ### Example
    /// ```javascript
    /// let a = []
    /// let b = false
    /// a == b
    /// ```
    Eqeqeq,
    eslint,
    pedantic,
    conditional_fix
);

impl Rule for Eqeqeq {
    fn from_configuration(value: serde_json::Value) -> Self {
        let first_arg = value.get(0).and_then(serde_json::Value::as_str);

        let null_type = value
            .get(usize::from(first_arg.is_some()))
            .and_then(|v| v.get("null"))
            .and_then(serde_json::Value::as_str)
            .map(NullType::from)
            .unwrap_or_default();

        let compare_type = first_arg.map(CompareType::from).unwrap_or_default();

        Self { compare_type, null_type }
    }

    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::BinaryExpression(binary_expr) = node.kind() else {
            return;
        };
        let is_null = is_null_check(binary_expr);
        let enforce_rule_for_null = matches!(self.null_type, NullType::Always);
        let enforce_inverse_rule_for_null = matches!(self.null_type, NullType::Never);

        if !matches!(binary_expr.operator, BinaryOperator::Equality | BinaryOperator::Inequality) {
            if enforce_inverse_rule_for_null && is_null {
                let operator = binary_expr.operator.as_str();
                // There are some uncontrolled cases to auto fix.
                // In ESlint, `null >= null` will be auto fixed to `null > null` which is also wrong.
                // So I just report it.
                ctx.diagnostic(eqeqeq_diagnostic(
                    operator,
                    &operator[0..operator.len() - 1],
                    binary_expr.span,
                ));
            }

            return;
        }

        let is_type_of_binary_bool = is_type_of_binary(binary_expr);
        let are_literals_and_same_type_bool =
            are_literals_and_same_type(&binary_expr.left, &binary_expr.right);
        // The "smart" option enforces the use of `===` and `!==` except for these cases:
        //  - Comparing two literal values
        //  - Evaluating the value of typeof
        //  - Comparing against null
        if matches!(self.compare_type, CompareType::Smart)
            && (is_type_of_binary_bool || are_literals_and_same_type_bool || is_null)
        {
            return;
        }

        if !enforce_rule_for_null && is_null {
            return;
        }

        let operator = binary_expr.operator.as_str();
        let (preferred_operator, preferred_operator_with_padding) =
            to_strict_eq_operator_str(binary_expr.operator);

        #[allow(clippy::cast_possible_truncation)]
        let operator_span = {
            let left_end = binary_expr.left.span().end;
            let right_start = binary_expr.right.span().start;
            let offset = Span::new(left_end, right_start)
                .source_text(ctx.source_text())
                .find(operator)
                .unwrap_or(0) as u32;

            let operator_start = left_end + offset;
            let operator_end = operator_start + operator.len() as u32;
            Span::new(operator_start, operator_end)
        };

        // If the comparison is a `typeof` comparison or both sides are literals with the same type, then it's safe to fix.
        if is_type_of_binary_bool || are_literals_and_same_type_bool {
            ctx.diagnostic_with_fix(
                eqeqeq_diagnostic(operator, preferred_operator, operator_span),
                |fixer| {
                    let start = binary_expr.left.span().end;
                    let end = binary_expr.right.span().start;
                    let span = Span::new(start, end);
                    fixer.replace(span, preferred_operator_with_padding)
                },
            );
        } else {
            ctx.diagnostic(eqeqeq_diagnostic(operator, preferred_operator, operator_span));
        }
    }
}

#[derive(Debug, Default, Clone)]
enum CompareType {
    #[default]
    Always,
    Smart,
}

impl CompareType {
    pub fn from(raw: &str) -> Self {
        match raw {
            "smart" => Self::Smart,
            _ => Self::Always,
        }
    }
}

#[derive(Debug, Default, Clone)]
enum NullType {
    #[default]
    Always,
    Never,
    Ignore,
}

impl NullType {
    pub fn from(raw: &str) -> Self {
        match raw {
            "always" => Self::Always,
            "never" => Self::Never,
            _ => Self::Ignore,
        }
    }
}

fn to_strict_eq_operator_str(operator: BinaryOperator) -> (&'static str, &'static str) {
    match operator {
        BinaryOperator::Equality => ("===", " === "),
        BinaryOperator::Inequality => ("!==", " !== "),
        _ => unreachable!(),
    }
}

/// Checks if either operand of a binary expression is a typeof operation
fn is_type_of_binary(binary_expr: &BinaryExpression) -> bool {
    match (&binary_expr.left, &binary_expr.right) {
        (Expression::UnaryExpression(unary_expr), _)
        | (_, Expression::UnaryExpression(unary_expr)) => {
            matches!(unary_expr.operator, UnaryOperator::Typeof)
        }
        _ => false,
    }
}

/// Checks if operands are literals of the same type
fn are_literals_and_same_type(left: &Expression, right: &Expression) -> bool {
    matches!(
        (left, right),
        (Expression::BooleanLiteral(_), Expression::BooleanLiteral(_))
            | (Expression::NullLiteral(_), Expression::NullLiteral(_))
            | (Expression::StringLiteral(_), Expression::StringLiteral(_))
            | (Expression::NumericLiteral(_), Expression::NumericLiteral(_))
            | (Expression::BigIntLiteral(_), Expression::BigIntLiteral(_))
            | (Expression::RegExpLiteral(_), Expression::RegExpLiteral(_))
            | (Expression::TemplateLiteral(_), Expression::TemplateLiteral(_))
    )
}

fn is_null_check(binary_expr: &BinaryExpression) -> bool {
    matches!(
        (&binary_expr.left, &binary_expr.right),
        (_, Expression::NullLiteral(_)) | (Expression::NullLiteral(_), _)
    )
}

#[test]
fn test() {
    use serde_json::json;

    use crate::tester::Tester;

    let pass = vec![
        ("typeof foo == 'undefined'", Some(json!(["smart"]))),
        ("'hello' != 'world'", Some(json!(["smart"]))),
        ("0 == 0", Some(json!(["smart"]))),
        ("true == true", Some(json!(["smart"]))),
        ("foo == null", Some(json!(["smart"]))),
        ("foo === null", None),
        // Always use === or !== with `null`
        ("null === null", Some(json!(["always", {"null": "always"}]))),
        // Never use === or !== with `null`
        ("null == null", Some(json!(["always", {"null": "never"}]))),
        // Do not apply this rule to `null`.
        ("null == null", Some(json!(["smart", {"null": "ignore"}]))),
        // Issue: <https://github.com/oxc-project/oxc/issues/8773>
        ("href != null", Some(json!([{"null": "ignore"}]))),
    ];

    let fail = vec![
        // ESLint will perform like below case
        ("null >= 1", Some(json!(["always", {"null": "never"}]))),
        ("typeof foo == 'undefined'", None),
        ("'hello' != 'world'", None),
        ("0 == 0", None),
        ("true == true", None),
        ("foo == null", None),
        ("a == b", None),
        ("foo == true", None),
        ("bananas != 1", None),
        ("value == undefined", None),
        ("null == null", Some(json!(["always", {"null": "always"}]))),
    ];

    let fix = vec![
        ("null==null", "null === null", None),
        ("'foo'=='foo'", "'foo' === 'foo'", None),
        ("typeof a == b", "typeof a === b", None),
        ("1000  !=  1000", "1000 !== 1000", None),
        // The following cases will not be fixed
        ("(1000 + 1)  !=  1000", "(1000 + 1)  !=  1000", None),
        ("a == b", "a == b", None),
    ];

    Tester::new(Eqeqeq::NAME, Eqeqeq::PLUGIN, pass, fail).expect_fix(fix).test_and_snapshot();
}
