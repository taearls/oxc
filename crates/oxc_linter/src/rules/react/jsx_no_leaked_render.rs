use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;
use rustc_hash::FxHashSet;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AstNode,
    context::LintContext,
    fixer::{RuleFix, RuleFixer},
    rule::{DefaultRuleConfig, Rule},
};

fn jsx_no_leaked_render_diagnostic(span: Span) -> OxcDiagnostic {
    // See <https://oxc.rs/docs/contribute/linter/adding-rules.html#diagnostics> for details
    OxcDiagnostic::warn("Should be an imperative statement about what is wrong.")
        .with_help("Should be a command-like statement that tells the user how to fix the issue.")
        .with_label(span)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
struct ConfigElement0 {
    valid_strategies: FxHashSet<ValidStrategies>,
    ignore_attributes: bool,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged, rename_all = "camelCase")]
enum ValidStrategies {}

#[derive(Debug, Default, Clone, Deserialize, Serialize, JsonSchema)]
pub struct JsxNoLeakedRender(ConfigElement0);

// See <https://github.com/oxc-project/oxc/issues/6050> for documentation details.
declare_oxc_lint!(
    /// ### What it does
    ///
    /// FIXME: Briefly describe the rule's purpose.
    ///
    /// ### Why is this bad?
    ///
    /// FIXME: Explain why violating this rule is problematic.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```jsx
    /// FIXME: Add at least one example of code that violates the rule.
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```jsx
    /// FIXME: Add at least one example of code that is allowed with the rule.
    /// ```
    JsxNoLeakedRender,
    react,
    nursery, // TODO: change category to `correctness`, `suspicious`, `pedantic`, `perf`, `restriction`, or `style`
             // See <https://oxc.rs/docs/contribute/linter.html#rule-category> for details
    pending, // TODO: describe fix capabilities. Remove or set to `none` if no fix can be done,
             // keep at 'pending' if you think one could be added but don't know how.
             // Options are 'fix', 'fix_dangerous', 'suggestion', and 'conditional_fix_suggestion'
    config = JsxNoLeakedRender,
    version = "next",
);

impl Rule for JsxNoLeakedRender {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        serde_json::from_value::<DefaultRuleConfig<Self>>(value).map(DefaultRuleConfig::into_inner)
    }

    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {}
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![];

    let fail = vec![
        ("
                    const Example = () => {
                      return (
                        <>
                          {0 && <Something/>}
                          {'' && <Something/>}
                          {NaN && <Something/>}
                        </>
                      )
                    }
                  ", None, Some(serde_json::json!({ "settings": { "react": { "version": "17.999.999" } } }))),
("
                    const Example = () => {
                      return (
                        <>
                          {0 && <Something/>}
                          {'' && <Something/>}
                          {NaN && <Something/>}
                        </>
                      )
                    }
                  ", None, Some(serde_json::json!({ "settings": { "react": { "version": "18.0.0" } } }))),
("
                    const Component = ({ count, title }) => {
                      return <div>{count && title}</div>
                    }
                  ", None, None),
("
                    const Component = ({ count }) => {
                      return <div>{count && <span>There are {count} results</span>}</div>
                    }
                  ", None, None),
("
                    const Component = ({ elements }) => {
                      return <div>{elements.length && <List elements={elements}/>}</div>
                    }
                  ", None, None),
("
                    const Component = ({ nestedCollection }) => {
                      return <div>{nestedCollection.elements.length && <List elements={nestedCollection.elements}/>}</div>
                    }
                  ", None, None),
("
                    const Component = ({ elements }) => {
                      return <div>{elements[0] && <List elements={elements}/>}</div>
                    }
                  ", None, None),
("
                    const Component = ({ numberA, numberB }) => {
                      return <div>{(numberA || numberB) && <Results>{numberA+numberB}</Results>}</div>
                    }
                  ", None, None),
("
                    const Component = ({ numberA, numberB }) => {
                      return <div>{(numberA || numberB) && <Results>{numberA+numberB}</Results>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce", "ternary"] }])), None),
("
                    const Component = ({ count, title }) => {
                      return <div>{count && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }])), None),
("
                    const Component = ({ count }) => {
                      return <div>{count && <span>There are {count} results</span>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }])), None),
("
                    const Component = ({ elements }) => {
                      return <div>{elements.length && <List elements={elements}/>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }])), None),
("
                    const Component = ({ nestedCollection }) => {
                      return <div>{nestedCollection.elements.length && <List elements={nestedCollection.elements}/>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }])), None),
("
                    const Component = ({ elements }) => {
                      return <div>{elements[0] && <List elements={elements}/>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }])), None),
("
                    const Component = ({ numberA, numberB }) => {
                      return <div>{(numberA || numberB) && <Results>{numberA+numberB}</Results>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }])), None),
("
                    const Component = ({ someCondition, title }) => {
                      return <div>{!someCondition && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }])), None),
("
                    const Component = ({ count, title }) => {
                      return <div>{!!count && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }])), None),
("
                    const Component = ({ count, title }) => {
                      return <div>{count > 0 && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }])), None),
("
                    const Component = ({ count, title }) => {
                      return <div>{0 != count && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }])), None),
("
                    const Component = ({ count, total, title }) => {
                      return <div>{count < total && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }])), None),
("
                    const Component = ({ count, title, somethingElse }) => {
                      return <div>{!!(count && somethingElse) && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }])), None),
("
                    const Component = ({ count, title }) => {
                      return <div>{count && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const Component = ({ count }) => {
                      return <div>{count && <span>There are {count} results</span>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const Component = ({ elements }) => {
                      return <div>{elements.length && <List elements={elements}/>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const Component = ({ nestedCollection }) => {
                      return <div>{nestedCollection.elements.length && <List elements={nestedCollection.elements}/>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const Component = ({ elements }) => {
                      return <div>{elements[0] && <List elements={elements}/>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const Component = ({ numberA, numberB }) => {
                      return <div>{(numberA || numberB) && <Results>{numberA+numberB}</Results>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const Component = ({ connection, hasError, hasErrorUpdate}) => {
                      return <div>{connection && (hasError || hasErrorUpdate)}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const Component = ({ count, title }) => {
                      return <div>{count ? title : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const Component = ({ count, title }) => {
                      return <div>{!count ? title : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const Component = ({ count, somethingElse, title }) => {
                      return <div>{count && somethingElse ? title : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const Component = ({ items, somethingElse, title }) => {
                      return <div>{items.length > 0 && somethingElse && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const MyComponent = () => {
                      const items = []
                      const breakpoint = { phones: true }
            
                      return <div>{items.length > 0 && breakpoint.phones && <span />}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce", "ternary"] }])), None),
("
                    const MyComponent = () => {
                      return <div>{maybeObject && (isFoo ? <Aaa /> : <Bbb />)}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const Component = ({ enabled, checked }) => {
                      return <CheckBox checked={enabled && checked} />
                    }
                  ", None, None),
("
                    const MyComponent = () => {
                      return <Something checked={isIndeterminate ? false : isChecked} />
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const MyComponent = () => {
                      return <Something checked={cond && isIndeterminate ? false : isChecked} />
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const isOpen = 0;
                    const Component = () => {
                      return <Popover open={isOpen && items.length > 0} />
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }])), None),
("
                    const Component = ({ enabled }) => {
                      return (
                        <Foo bar={
                          <Something>{enabled && <MuchWow />}</Something>
                        } />
                      )
                    }
                  ", Some(serde_json::json!([{ "ignoreAttributes": true }])), None)
    ];

    let fix = vec![
        ("
                    const Example = () => {
                      return (
                        <>
                          {0 && <Something/>}
                          {'' && <Something/>}
                          {NaN && <Something/>}
                        </>
                      )
                    }
                  ", "
                    const Example = () => {
                      return (
                        <>
                          {0 ? <Something/> : null}
                          {'' ? <Something/> : null}
                          {NaN ? <Something/> : null}
                        </>
                      )
                    }
                  ", None),
("
                    const Example = () => {
                      return (
                        <>
                          {0 && <Something/>}
                          {'' && <Something/>}
                          {NaN && <Something/>}
                        </>
                      )
                    }
                  ", "
                    const Example = () => {
                      return (
                        <>
                          {0 ? <Something/> : null}
                          {'' && <Something/>}
                          {NaN ? <Something/> : null}
                        </>
                      )
                    }
                  ", None),
("
                    const Component = ({ count, title }) => {
                      return <div>{count && title}</div>
                    }
                  ", "
                    const Component = ({ count, title }) => {
                      return <div>{count ? title : null}</div>
                    }
                  ", None),
("
                    const Component = ({ count }) => {
                      return <div>{count && <span>There are {count} results</span>}</div>
                    }
                  ", "
                    const Component = ({ count }) => {
                      return <div>{count ? <span>There are {count} results</span> : null}</div>
                    }
                  ", None),
("
                    const Component = ({ elements }) => {
                      return <div>{elements.length && <List elements={elements}/>}</div>
                    }
                  ", "
                    const Component = ({ elements }) => {
                      return <div>{elements.length ? <List elements={elements}/> : null}</div>
                    }
                  ", None),
("
                    const Component = ({ nestedCollection }) => {
                      return <div>{nestedCollection.elements.length && <List elements={nestedCollection.elements}/>}</div>
                    }
                  ", "
                    const Component = ({ nestedCollection }) => {
                      return <div>{nestedCollection.elements.length ? <List elements={nestedCollection.elements}/> : null}</div>
                    }
                  ", None),
("
                    const Component = ({ elements }) => {
                      return <div>{elements[0] && <List elements={elements}/>}</div>
                    }
                  ", "
                    const Component = ({ elements }) => {
                      return <div>{elements[0] ? <List elements={elements}/> : null}</div>
                    }
                  ", None),
("
                    const Component = ({ numberA, numberB }) => {
                      return <div>{(numberA || numberB) && <Results>{numberA+numberB}</Results>}</div>
                    }
                  ", "
                    const Component = ({ numberA, numberB }) => {
                      return <div>{(numberA || numberB) ? <Results>{numberA+numberB}</Results> : null}</div>
                    }
                  ", None),
("
                    const Component = ({ numberA, numberB }) => {
                      return <div>{(numberA || numberB) && <Results>{numberA+numberB}</Results>}</div>
                    }
                  ", "
                    const Component = ({ numberA, numberB }) => {
                      return <div>{!!(numberA || numberB) && <Results>{numberA+numberB}</Results>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce", "ternary"] }]))),
("
                    const Component = ({ count, title }) => {
                      return <div>{count && title}</div>
                    }
                  ", "
                    const Component = ({ count, title }) => {
                      return <div>{count ? title : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }]))),
("
                    const Component = ({ count }) => {
                      return <div>{count && <span>There are {count} results</span>}</div>
                    }
                  ", "
                    const Component = ({ count }) => {
                      return <div>{count ? <span>There are {count} results</span> : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }]))),
("
                    const Component = ({ elements }) => {
                      return <div>{elements.length && <List elements={elements}/>}</div>
                    }
                  ", "
                    const Component = ({ elements }) => {
                      return <div>{elements.length ? <List elements={elements}/> : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }]))),
("
                    const Component = ({ nestedCollection }) => {
                      return <div>{nestedCollection.elements.length && <List elements={nestedCollection.elements}/>}</div>
                    }
                  ", "
                    const Component = ({ nestedCollection }) => {
                      return <div>{nestedCollection.elements.length ? <List elements={nestedCollection.elements}/> : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }]))),
("
                    const Component = ({ elements }) => {
                      return <div>{elements[0] && <List elements={elements}/>}</div>
                    }
                  ", "
                    const Component = ({ elements }) => {
                      return <div>{elements[0] ? <List elements={elements}/> : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }]))),
("
                    const Component = ({ numberA, numberB }) => {
                      return <div>{(numberA || numberB) && <Results>{numberA+numberB}</Results>}</div>
                    }
                  ", "
                    const Component = ({ numberA, numberB }) => {
                      return <div>{(numberA || numberB) ? <Results>{numberA+numberB}</Results> : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }]))),
("
                    const Component = ({ someCondition, title }) => {
                      return <div>{!someCondition && title}</div>
                    }
                  ", "
                    const Component = ({ someCondition, title }) => {
                      return <div>{!someCondition ? title : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }]))),
("
                    const Component = ({ count, title }) => {
                      return <div>{!!count && title}</div>
                    }
                  ", "
                    const Component = ({ count, title }) => {
                      return <div>{count ? title : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }]))),
("
                    const Component = ({ count, title }) => {
                      return <div>{count > 0 && title}</div>
                    }
                  ", "
                    const Component = ({ count, title }) => {
                      return <div>{count > 0 ? title : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }]))),
("
                    const Component = ({ count, title }) => {
                      return <div>{0 != count && title}</div>
                    }
                  ", "
                    const Component = ({ count, title }) => {
                      return <div>{0 != count ? title : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }]))),
("
                    const Component = ({ count, total, title }) => {
                      return <div>{count < total && title}</div>
                    }
                  ", "
                    const Component = ({ count, total, title }) => {
                      return <div>{count < total ? title : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }]))),
("
                    const Component = ({ count, title, somethingElse }) => {
                      return <div>{!!(count && somethingElse) && title}</div>
                    }
                  ", "
                    const Component = ({ count, title, somethingElse }) => {
                      return <div>{count && somethingElse ? title : null}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["ternary"] }]))),
("
                    const Component = ({ count, title }) => {
                      return <div>{count && title}</div>
                    }
                  ", "
                    const Component = ({ count, title }) => {
                      return <div>{!!count && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const Component = ({ count }) => {
                      return <div>{count && <span>There are {count} results</span>}</div>
                    }
                  ", "
                    const Component = ({ count }) => {
                      return <div>{!!count && <span>There are {count} results</span>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const Component = ({ elements }) => {
                      return <div>{elements.length && <List elements={elements}/>}</div>
                    }
                  ", "
                    const Component = ({ elements }) => {
                      return <div>{!!elements.length && <List elements={elements}/>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const Component = ({ nestedCollection }) => {
                      return <div>{nestedCollection.elements.length && <List elements={nestedCollection.elements}/>}</div>
                    }
                  ", "
                    const Component = ({ nestedCollection }) => {
                      return <div>{!!nestedCollection.elements.length && <List elements={nestedCollection.elements}/>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const Component = ({ elements }) => {
                      return <div>{elements[0] && <List elements={elements}/>}</div>
                    }
                  ", "
                    const Component = ({ elements }) => {
                      return <div>{!!elements[0] && <List elements={elements}/>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const Component = ({ numberA, numberB }) => {
                      return <div>{(numberA || numberB) && <Results>{numberA+numberB}</Results>}</div>
                    }
                  ", "
                    const Component = ({ numberA, numberB }) => {
                      return <div>{!!(numberA || numberB) && <Results>{numberA+numberB}</Results>}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const Component = ({ connection, hasError, hasErrorUpdate}) => {
                      return <div>{connection && (hasError || hasErrorUpdate)}</div>
                    }
                  ", "
                    const Component = ({ connection, hasError, hasErrorUpdate}) => {
                      return <div>{!!connection && (hasError || hasErrorUpdate)}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const Component = ({ count, title }) => {
                      return <div>{count ? title : null}</div>
                    }
                  ", "
                    const Component = ({ count, title }) => {
                      return <div>{!!count && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const Component = ({ count, title }) => {
                      return <div>{!count ? title : null}</div>
                    }
                  ", "
                    const Component = ({ count, title }) => {
                      return <div>{!count && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const Component = ({ count, somethingElse, title }) => {
                      return <div>{count && somethingElse ? title : null}</div>
                    }
                  ", "
                    const Component = ({ count, somethingElse, title }) => {
                      return <div>{!!count && !!somethingElse && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const Component = ({ items, somethingElse, title }) => {
                      return <div>{items.length > 0 && somethingElse && title}</div>
                    }
                  ", "
                    const Component = ({ items, somethingElse, title }) => {
                      return <div>{items.length > 0 && !!somethingElse && title}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const MyComponent = () => {
                      const items = []
                      const breakpoint = { phones: true }
            
                      return <div>{items.length > 0 && breakpoint.phones && <span />}</div>
                    }
                  ", "
                    const MyComponent = () => {
                      const items = []
                      const breakpoint = { phones: true }
            
                      return <div>{items.length > 0 && !!breakpoint.phones && <span />}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce", "ternary"] }]))),
("
                    const MyComponent = () => {
                      return <div>{maybeObject && (isFoo ? <Aaa /> : <Bbb />)}</div>
                    }
                  ", "
                    const MyComponent = () => {
                      return <div>{!!maybeObject && (isFoo ? <Aaa /> : <Bbb />)}</div>
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const Component = ({ enabled, checked }) => {
                      return <CheckBox checked={enabled && checked} />
                    }
                  ", "
                    const Component = ({ enabled, checked }) => {
                      return <CheckBox checked={enabled ? checked : null} />
                    }
                  ", None),
("
                    const isOpen = 0;
                    const Component = () => {
                      return <Popover open={isOpen && items.length > 0} />
                    }
                  ", "
                    const isOpen = 0;
                    const Component = () => {
                      return <Popover open={!!isOpen && items.length > 0} />
                    }
                  ", Some(serde_json::json!([{ "validStrategies": ["coerce"] }]))),
("
                    const Component = ({ enabled }) => {
                      return (
                        <Foo bar={
                          <Something>{enabled && <MuchWow />}</Something>
                        } />
                      )
                    }
                  ", "
                    const Component = ({ enabled }) => {
                      return (
                        <Foo bar={
                          <Something>{enabled ? <MuchWow /> : null}</Something>
                        } />
                      )
                    }
                  ", Some(serde_json::json!([{ "ignoreAttributes": true }])))
    ];

    Tester::new(JsxNoLeakedRender::NAME, JsxNoLeakedRender::PLUGIN, pass, fail)
        .expect_fix(fix)
        .test_and_snapshot();
}
