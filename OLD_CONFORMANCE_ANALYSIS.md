# Formatter Conformance Test Regression Analysis

## Executive Summary

After fresh comparison with main branch, we have **12 specific JS tests** that need fixing. These are all related to the removal of `AstKind::Argument` which prevents the formatter from knowing when expressions are function arguments.

**Current Status:**
- We're at 493/699 JS tests passing (70.53%)
- Main branch has 505/699 passing (72.25%)  
- **Gap to close: 12 tests**

**Good news:** 7-8 of these tests appear to be simple fixes similar to what we've already successfully done.

**⚡ Quick Start:** Jump to "Revised Implementation Priorities" section for the ordered fix list.

## Test Results Timeline

| Branch/State | JavaScript Tests | TypeScript Tests | Notes |
|-------------|------------------|------------------|-------|
| **Main branch (baseline)** | 505/699 (72.25%) | 249/573 (43.46%) | Before AstKind::Argument removal |
| **Dev branch (regression)** | 464/699 (66.38%) | 244/573 (42.58%) | After removal, 46 tests regressed |
| **After Priority 1 Fix** | 490/699 (70.10%) | 246/573 (42.93%) | +28 tests recovered |
| **Current Status** | 490/699 (70.10%) | 246/573 (42.93%) | 18 tests still need fixes |

## Regression Impact (Updated from Fresh Comparison)
- **Original Regressions:** 46 tests (41 JS + 5 TS) 
- **Fixed with our changes:** 31 tests (29 JS + 2 TS) ✅
- **Actual Current Regressions from main:** 12 JS tests + 3 TS tests
- **Main branch baseline:** JS 505/699 (72.25%), TS 249/573 (43.46%)
- **Current branch status:** JS 493/699 (70.53%), TS 246/573 (42.93%)

## Current Test Regressions (12 JS Tests to Fix)

### Detailed Analysis of Each Regression

| Test File | Match % | Likely Issue | Complexity |
|-----------|---------|--------------|------------|
| **js/new-expression/call.js** | 75% | New expressions being called need parentheses logic (e.g., `new (Factory())()`) | ⭐ Low |
| **js/require/require.js** | 93.51% | Require calls with conditional/ternary arguments - minor formatting | ⭐ Low |
| **js/arrows/currying-4.js** | 78.15% | Curried arrow functions as arguments - parentheses/indentation | ⭐⭐ Medium |
| **js/test-declarations/angular_async.js** | 86.21% | Test framework calls with function arguments | ⭐⭐ Medium |
| **js/test-declarations/angular_fakeAsync.js** | 75.86% | Similar to angular_async | ⭐⭐ Medium |
| **js/test-declarations/angular_waitForAsync.js** | 75.86% | Similar to angular_async | ⭐⭐ Medium |
| **js/functional-composition/pipe-function-calls.js** | 87.80% | Function composition/piping - argument context needed | ⭐⭐ Medium |
| **js/arrows/currying-2.js** | 59.08% | Complex curried arrow functions - multiple formatting issues | ⭐⭐ Medium |
| **js/arrows/chain-as-arg.js** | 43.59% | Arrow functions with long params in method chains | ⭐⭐⭐ High |
| **js/method-chain/print-width-120/constructor.js** | 71.43% | Method chaining with print width constraints | ⭐⭐⭐ High |
| **js/preserve-line/parameter-list.js** | 97.37% | Line preservation in parameter lists (very close but complex) | ⭐⭐⭐ High |
| **js/function/issue-10277.js** | 30.77% | Specific complex function formatting issue | ⭐⭐⭐ High |

### Root Cause Analysis

All these regressions stem from the removal of `AstKind::Argument`, which means the formatter can no longer determine when expressions are used as function arguments. This affects:

1. **Parentheses decisions** - Expressions need different parentheses when used as arguments
2. **Line breaking** - Arguments have special line breaking rules  
3. **Indentation** - Argument context affects indentation levels
4. **Grouping** - Multi-line arguments need special grouping

## Revised Implementation Priorities

### 🟢 Priority 1: Simple Parentheses Fixes (Low Complexity)
These are similar to the successful object/class expression fixes:

#### 1.1 New Expression Calls ⭐
- **Test:** `js/new-expression/call.js` (75% match)
- **Fix:** Add parentheses logic for new expressions used as callees
- **Pattern:** Similar to class expression fix - check if new expression is being called
- **Expected impact:** 1 test fixed

#### 1.2 Require Statement Arguments ⭐  
- **Test:** `js/require/require.js` (93.51% match)
- **Fix:** Minor formatting in require() arguments with conditionals
- **Pattern:** Check argument context in require calls
- **Expected impact:** 1 test fixed

### 🟡 Priority 2: Arrow Functions as Arguments (Medium Complexity)
These need argument context for proper formatting:

#### 2.1 Test Declaration Functions ⭐⭐
- **Tests:** `js/test-declarations/angular_*.js` (3 tests, 75-86% match)
- **Fix:** Detect when functions are test declaration arguments
- **Pattern:** Special handling for known test framework functions
- **Expected impact:** 3 tests fixed

#### 2.2 Curried Arrow Functions ⭐⭐
- **Tests:** `js/arrows/currying-2.js`, `js/arrows/currying-4.js` (59-78% match)
- **Fix:** Arrow function formatting when used as arguments
- **Pattern:** Check parent context for arrow expressions
- **Expected impact:** 2 tests fixed

#### 2.3 Function Composition ⭐⭐
- **Test:** `js/functional-composition/pipe-function-calls.js` (87.80% match)
- **Fix:** Function calls in composition patterns
- **Pattern:** Detect pipe/compose patterns
- **Expected impact:** 1 test fixed

### 🔴 Priority 3: Complex Formatting Issues (High Complexity)

#### 3.1 Method Chaining with Arguments ⭐⭐⭐
- **Tests:** `js/arrows/chain-as-arg.js`, `js/method-chain/print-width-120/constructor.js`
- **Issue:** Complex interaction of method chaining, line width, and argument formatting
- **Complexity:** Requires understanding print width constraints and chaining

#### 3.2 Line Preservation ⭐⭐⭐
- **Test:** `js/preserve-line/parameter-list.js` (97.37% match - so close!)
- **Issue:** Preserving user's blank lines in parameter lists
- **Complexity:** Line preservation is architecturally complex

#### 3.3 Specific Edge Cases ⭐⭐⭐
- **Test:** `js/function/issue-10277.js` (30.77% match)
- **Issue:** Specific complex case, needs investigation
- **Complexity:** Low match % suggests fundamental issue

## Recommended Action Plan

**Start with Priority 1** - These should be quick wins similar to what we've already done:
1. Fix new expression calls (1.1)
2. Fix require statements (1.2)

**Then tackle Priority 2** - These need argument context but are manageable:
3. Fix test declarations (2.1) - 3 tests at once
4. Fix curried arrows (2.2) - Good learning for arrow handling
5. Fix function composition (2.3)

**Defer Priority 3** - These need architectural changes or deep investigation

This approach should recover 7-8 tests relatively quickly, bringing us from 493 to ~500-501 JS tests passing.

## Quick Start for Developers

### Prerequisites
1. Checkout the `temp/fix-conformance-tests-take-two` branch
2. Familiarize yourself with the project commands in `CLAUDE.md`
3. Key commands you'll use:
   ```bash
   just ready          # Run full CI pipeline before starting
   just coverage       # Run conformance tests to see current status
   just test           # Run tests after making changes
   just ast            # Regenerate AST if you modify AST definitions
   ```

### Understanding the Problem
The `AstKind::Argument` enum was removed from the AST, breaking the formatter's ability to detect when expressions are function arguments. This affects:
- Parentheses placement (main issue)
- Comment positioning
- Line breaking decisions
- Trailing comma handling

### How to Fix Issues (Pattern from Successful Fix)
1. **Identify the formatting rule** - Find the `NeedsParentheses` or formatting implementation for the affected node type
2. **Check for existing utilities** - Look in `crates/oxc_formatter/src/utils/` for helper functions
3. **Use parent context** - Most fixes involve checking the parent node type to make formatting decisions
4. **Test incrementally** - Run specific test: 
   ```bash
   # Test a specific file
   cargo run -p oxc_prettier_conformance -- --filter "js/arrows/semi/semi.js"
   
   # See the actual diff
   cargo test -p oxc_prettier_conformance 2>&1 | grep -A10 "js/arrows/semi"
   ```
5. **Commit each fix separately** - After verifying a fix works:
   ```bash
   # Check what files changed
   git status
   git diff path/to/changed/file.rs
   
   # Stage ONLY the files related to this fix
   git add crates/oxc_formatter/src/specific/file.rs
   
   # Create descriptive commit
   git commit -m "fix(formatter): brief description of what was fixed
   
   Detailed explanation of the problem and solution.
   This fixes N conformance tests.
   
   🤖 Generated with [Claude Code](https://claude.ai/code)
   
   Co-Authored-By: Claude <noreply@anthropic.com>"
   ```

### Example of Successful Fix
```rust
// Before: Object literals always got parentheses
impl<'a> NeedsParentheses<'a> for AstNode<'a, ObjectExpression<'a>> {
    fn needs_parentheses(&self, f: &Formatter<'_, 'a>) -> bool {
        is_class_extends(parent, self.span()) || 
        is_first_in_statement(...)
    }
}

// After: Check if it's a function argument first
impl<'a> NeedsParentheses<'a> for AstNode<'a, ObjectExpression<'a>> {
    fn needs_parentheses(&self, f: &Formatter<'_, 'a>) -> bool {
        // Don't add parens for function arguments
        if is_expression_used_as_call_argument(self.span, parent) {
            return false;
        }
        is_class_extends(parent, self.span()) || 
        is_first_in_statement(...)
    }
}
```

## Git Workflow for Fixes

### Important: Commit Each Fix Separately

To maintain a clean git history and make it easier to debug/revert if needed, follow this workflow:

#### 1. Before Starting a Fix
```bash
git status  # Ensure working directory is clean
git diff    # Make sure no uncommitted changes
```

#### 2. After Implementing and Testing a Fix
```bash
# Run conformance to verify improvement
just coverage | grep "compatibility:"

# Run specific test to confirm it passes
cargo run -p oxc_prettier_conformance -- --filter "js/specific/test.js"

# Check what changed
git status
git diff

# Stage ONLY files for this specific fix
git add crates/oxc_formatter/src/[specific-file].rs
# Do NOT stage snapshots or unrelated files
```

#### 3. Create Descriptive Commit
```bash
git commit -m "fix(formatter): [specific issue fixed]

[Detailed explanation of what was wrong and how it was fixed]

This fixes [N] conformance tests:
- [list specific tests if known]

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

#### 4. Files to Exclude from Commits
- **Never commit:** Snapshot files (`snapshots/*.snap.md`) - these regenerate automatically
- **Never commit:** Analysis documents (`REMAINING_CONFORMANCE_TEST_ANALYSIS.md`, etc.)
- **Never commit:** Editor config files (`.cursor/`, `.vscode/`)
- **Only commit:** The actual source files you modified for the fix

#### 5. Example of Good Commit
```bash
# After fixing class expression parentheses
git add crates/oxc_formatter/src/parentheses/expression.rs
git commit -m "fix(formatter): don't add parentheses to class expressions as function arguments

Class expressions were incorrectly getting parentheses when passed as
arguments. Applied the same pattern as object literals using the
is_expression_used_as_call_argument utility.

This fixes 3 JS conformance tests including js/classes/call.js

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

#### 6. If You Make a Mistake
```bash
# If you committed too many files
git reset --soft HEAD~1  # Undo last commit but keep changes
git status               # Re-stage only the correct files
git add [correct-file]
git commit -m "..."

# If you need to amend the commit message
git commit --amend

# If you need to add a forgotten file to the last commit
git add [forgotten-file]
git commit --amend --no-edit
```

## JavaScript Tests Failing in Dev Branch (Original 41 Failures)

⚠️ **Note**: This list shows the ORIGINAL 41 failures. After the Priority 1 fix, 26 of these are now passing. Run `just coverage` to see current failures.

The following tests were originally failing in the dev branch:

### Arrow Functions
- `js/arrow-call/arrow_call.js` (89.80% match)
- `js/arrows/call.js` (91.01% match)
- `js/arrows/chain-as-arg.js` (43.59% match)
- `js/arrows/comment.js` (78.11% match)
- `js/arrows/curried.js` (72.58% match)
- `js/arrows/currying-2.js` (59.08% match)
- `js/arrows/currying-4.js` (78.15% match)
- `js/arrows/long-contents.js` (75.00% match)
- `js/arrows/semi/semi.js` (0.00% match)

### Assignment and Expressions
- `js/assignment/issue-10218.js` (52.63% match)
- `js/assignment/issue-7572.js` (72.73% match)
- `js/assignment/sequence.js` (71.43% match)
- `js/async/inline-await.js` (25.00% match)
- `js/async/nested.js` (16.67% match)
- `js/binary-expressions/inline-jsx.js` (40.00% match)
- `js/binary-expressions/jsx_parent.js` (33.85% match)
- `js/binary-expressions/return.js` (90.00% match)

### Function Calls and Method Chains
- `js/break-calls/break.js` (87.23% match)
- `js/call/first-argument-expansion/issue-4401.js` (87.50% match)
- `js/call/first-argument-expansion/jsx.js` (0.00% match)
- `js/call/first-argument-expansion/test.js` (98.76% match)
- `js/chain-expression/issue-15785-3.js` (50.00% match)

### Classes
- `js/class-comment/class-property.js` (30.77% match)
- `js/class-comment/misc.js` (72.73% match)
- `js/class-comment/superclass.js` (57.83% match)
- `js/class-static-block/class-static-block.js` (57.14% match)
- `js/class-static-block/with-line-breaks.js` (50.00% match)
- `js/classes/assignment.js` (81.25% match)
- `js/classes/call.js` (0.00% match)
- `js/classes/new.js` (50.00% match)
- `js/classes/property.js` (62.86% match)
- `js/classes-private-fields/with_comments.js` (30.77% match)

### Comments (Major Category)
- `js/comments/15661.js` (53.63% match)
- `js/comments/16398.js` (80.00% match)
- `js/comments/blank.js` (95.24% match)
- `js/comments/dangling_array.js` (40.00% match)
- `js/comments/dynamic_imports.js` (71.43% match)
- `js/comments/empty-statements.js` (90.91% match)
- `js/comments/export.js` (97.37% match)
- `js/comments/function-declaration.js` (91.06% match)
- `js/comments/issue-3532.js` (79.73% match)
- `js/comments/issues.js` (93.43% match)
- `js/comments/jsdoc-nestled-dangling.js` (93.02% match)
- `js/comments/jsdoc-nestled.js` (89.29% match)
- `js/comments/jsdoc.js` (47.83% match)
- `js/comments/jsx.js` (41.63% match)
- `js/comments/last-arg.js` (80.65% match)
- `js/comments/return-statement.js` (89.35% match)
- `js/comments/tagged-template-literal.js` (69.23% match)
- `js/comments/template-literal.js` (30.43% match)
- `js/comments/trailing_space.js` (60.00% match)
- `js/comments/variable_declarator.js` (49.31% match)

### Comments - Closure Typecast
- `js/comments-closure-typecast/binary-expr.js` (0.00% match)
- `js/comments-closure-typecast/closure-compiler-type-cast.js` (66.13% match)
- `js/comments-closure-typecast/comment-in-the-middle.js` (90.91% match)
- `js/comments-closure-typecast/comment-placement.js` (61.54% match)
- `js/comments-closure-typecast/extra-spaces-and-asterisks.js` (0.00% match)
- `js/comments-closure-typecast/iife.js` (27.27% match)
- `js/comments-closure-typecast/issue-4124.js` (47.37% match)
- `js/comments-closure-typecast/issue-8045.js` (59.26% match)
- `js/comments-closure-typecast/issue-9358.js` (16.00% match)
- `js/comments-closure-typecast/member.js` (0.00% match)
- `js/comments-closure-typecast/nested.js` (23.53% match)
- `js/comments-closure-typecast/object-with-comment.js` (38.10% match)
- `js/comments-closure-typecast/satisfies.js` (33.33% match)
- `js/comments-closure-typecast/superclass.js` (0.00% match)
- `js/comments-closure-typecast/ways-to-specify-type.js` (15.38% match)

### Flow Types
- `js/comments/flow-types/inline.js` (62.50% match)

### Conditional Expressions and Ternaries
- `js/conditional/comments.js` (60.35% match)
- `js/conditional/new-ternary-examples.js` (43.38% match)
- `js/conditional/new-ternary-spec.js` (58.16% match)
- `js/conditional/postfix-ternary-regressions.js` (65.17% match)
- `js/ternaries/binary.js` (43.27% match)
- `js/ternaries/func-call.js` (50.56% match)
- `js/ternaries/indent-after-paren.js` (52.41% match)
- `js/ternaries/indent.js` (6.16% match)
- `js/ternaries/nested-in-condition.js` (23.90% match)
- `js/ternaries/nested.js` (25.63% match)
- `js/ternaries/parenthesis.js` (29.31% match)
- `js/ternaries/test.js` (32.79% match)
- `js/ternaries/parenthesis/await-expression.js` (14.29% match)

### Decorators
- `js/decorators/classes.js` (73.68% match)
- `js/decorators/comments.js` (73.33% match)
- `js/decorators/member-expression.js` (80.60% match)
- `js/decorators/mobx.js` (70.45% match)
- `js/decorators/multiline.js` (44.44% match)
- `js/decorators/multiple.js` (61.54% match)
- `js/decorators/parens.js` (75.00% match)
- `js/decorators/class-expression/arguments.js` (50.00% match)
- `js/decorators/class-expression/class-expression.js` (55.56% match)
- `js/decorators/class-expression/member-expression.js` (0.00% match)
- `js/decorators/class-expression/super-class.js` (14.29% match)

### Object Handling
- `js/destructuring-ignore/ignore.js` (83.33% match)
- `js/object-multiline/multiline.js` (22.22% match)
- `js/object-prop-break-in/short-keys.js` (63.41% match)
- `js/object-prop-break-in/test.js` (88.57% match)
- `js/object-property-comment/after-key.js` (85.71% match)
- `js/object-property-ignore/ignore.js` (84.78% match)
- `js/object-property-ignore/issue-5678.js` (52.50% match)
- `js/objects/right-break.js` (70.27% match)

### Semicolons and No-Semi
- `js/no-semi/class.js` (46.55% match)
- `js/no-semi/comments.js` (36.36% match)
- `js/no-semi/issue2006.js` (37.50% match)
- `js/no-semi/no-semi.js` (90.66% match)

### Quotes and Strings
- `js/quote-props/classes.js` (47.06% match)
- `js/quote-props/objects.js` (93.14% match)
- `js/quote-props/with_numbers.js` (46.43% match)
- `js/quotes/objects.js` (80.00% match)
- `js/strings/escaped.js` (73.68% match)
- `js/strings/template-literals.js` (50.00% match)

### Template Literals
- `js/template/comment.js` (23.08% match)
- `js/template/graphql.js` (75.00% match)
- `js/template/indent.js` (85.71% match)
- `js/template-align/indent.js` (14.47% match)
- `js/template-literals/binary-exporessions.js` (0.00% match)
- `js/template-literals/conditional-expressions.js` (0.00% match)
- `js/template-literals/expressions.js` (88.89% match)
- `js/template-literals/indention.js` (51.16% match)
- `js/template-literals/logical-expressions.js` (0.00% match)
- `js/template-literals/sequence-expressions.js` (0.00% match)

### Test Declarations
- `js/test-declarations/angular_async.js` (86.21% match)
- `js/test-declarations/angular_fakeAsync.js` (75.86% match)
- `js/test-declarations/angular_waitForAsync.js` (75.86% match)
- `js/test-declarations/angularjs_inject.js` (69.84% match)
- `js/test-declarations/jest-each-template-string.js` (27.78% match)
- `js/test-declarations/jest-each.js` (67.65% match)
- `js/test-declarations/test_declarations.js` (73.40% match)

### Trailing Commas
- `js/trailing-comma/dynamic-import.js` (0.00% match)
- `js/trailing-comma/function-calls.js` (90.91% match)
- `js/trailing-comma/jsx.js` (0.00% match)
- `js/trailing-comma/trailing_whitespace.js` (97.67% match)

### JSX Tests
- `jsx/attr-element/attr-element.js` (0.00% match)
- `jsx/binary-expressions/relational-operators.js` (78.95% match)
- `jsx/comments/eslint-disable.js` (0.00% match)
- `jsx/comments/in-end-tag.js` (37.50% match)
- `jsx/comments/in-tags.js` (46.81% match)
- `jsx/comments/jsx-tag-comment-after-prop.js` (42.11% match)
- `jsx/comments/like-a-comment-in-jsx-text.js` (0.00% match)
- `jsx/deprecated-jsx-bracket-same-line-option/jsx.js` (45.33% match)
- `jsx/escape/nbsp.js` (70.00% match)
- `jsx/expression-with-types/expression.js` (0.00% match)
- `jsx/fbt/test.js` (44.00% match)
- `jsx/fragment/fragment.js` (78.57% match)
- `jsx/ignore/jsx_ignore.js` (67.31% match)
- `jsx/jsx/array-iter.js` (23.64% match)
- `jsx/jsx/arrow.js` (32.26% match)
- `jsx/jsx/attr-comments.js` (0.00% match)
- `jsx/jsx/await.js` (15.00% match)
- `jsx/jsx/conditional-expression.js` (74.84% match)
- `jsx/jsx/expression.js` (36.36% match)
- `jsx/jsx/flow_fix_me.js` (0.00% match)
- `jsx/jsx/html_escape.js` (16.67% match)
- `jsx/jsx/hug.js` (33.33% match)
- `jsx/jsx/logical-expression.js` (36.92% match)
- `jsx/jsx/object-property.js` (52.63% match)
- `jsx/jsx/open-break.js` (21.43% match)
- `jsx/jsx/parens.js` (50.00% match)
- `jsx/jsx/quotes.js` (42.55% match)
- `jsx/jsx/regex.js` (75.00% match)
- `jsx/jsx/return-statement.js` (74.47% match)
- `jsx/jsx/spacing.js` (40.00% match)
- `jsx/jsx/template-literal-in-attr.js` (26.67% match)
- `jsx/last-line/last_line.js` (44.51% match)
- `jsx/last-line/single_prop_multiline_string.js` (22.88% match)
- `jsx/multiline-assign/test.js` (28.00% match)
- `jsx/newlines/test.js` (35.29% match)
- `jsx/newlines/windows.js` (0.00% match)
- `jsx/optional-chaining/optional-chaining.jsx` (30.00% match)
- `jsx/significant-space/comments.js` (33.33% match)
- `jsx/significant-space/test.js` (26.52% match)
- `jsx/single-attribute-per-line/single-attribute-per-line.js` (57.46% match)
- `jsx/split-attrs/test.js` (12.73% match)
- `jsx/spread/attribute.js` (35.71% match)
- `jsx/spread/child.js` (33.33% match)
- `jsx/stateless-arrow-fn/test.js` (23.39% match)
- `jsx/text-wrap/issue-16897.js` (56.00% match)
- `jsx/text-wrap/test.js` (33.71% match)

### Miscellaneous
- `js/directives/test.js` (86.36% match)
- `js/explicit-resource-management/valid-await-using-comments.js` (91.89% match)
- `js/explicit-resource-management/valid-for-await-using-binding-escaped-of-of.js` (50.00% match)
- `js/explicit-resource-management/valid-for-using-binding-escaped-of-of.js` (50.00% match)
- `js/export-default/function_in_template.js` (0.00% match)
- `js/for/for-in-with-initializer.js` (37.50% match)
- `js/for/parentheses.js` (96.00% match)
- `js/function/issue-10277.js` (30.77% match)
- `js/function-first-param/function_expression.js` (76.67% match)
- `js/functional-composition/pipe-function-calls.js` (87.80% match)
- `js/functional-composition/ramda_compose.js` (95.74% match)
- `js/identifier/for-of/await.js` (50.00% match)
- `js/identifier/for-of/let.js` (69.23% match)
- `js/identifier/parentheses/let.js` (79.55% match)
- `js/import-assertions/keyword-detect.js` (71.43% match)
- `js/import-assertions/not-import-assertions.js` (50.00% match)
- `js/import-attributes/keyword-detect.js` (71.43% match)
- `js/import-attributes/long-sources.js` (75.00% match)
- `js/label/comment.js` (53.33% match)
- `js/last-argument-expansion/assignment-pattern.js` (87.50% match)
- `js/last-argument-expansion/break-parent.js` (78.26% match)
- `js/last-argument-expansion/dangling-comment-in-arrow-function.js` (22.22% match)
- `js/last-argument-expansion/edge_case.js` (83.97% match)
- `js/last-argument-expansion/empty-object.js` (75.86% match)
- `js/last-argument-expansion/issue-10708.js` (87.50% match)
- `js/last-argument-expansion/issue-7518.js` (85.71% match)
- `js/last-argument-expansion/jsx.js` (25.00% match)
- `js/last-argument-expansion/overflow.js` (87.15% match)
- `js/line-suffix-boundary/boundary.js` (30.43% match)
- `js/logical_expressions/issue-7024.js` (66.67% match)
- `js/member/expand.js` (86.96% match)
- `js/method-chain/break-last-call.js` (92.41% match)
- `js/method-chain/break-last-member.js` (80.56% match)
- `js/method-chain/comment.js` (97.56% match)
- `js/method-chain/computed.js` (66.67% match)
- `js/method-chain/conditional.js` (85.19% match)
- `js/method-chain/d3.js` (71.43% match)
- `js/method-chain/first_long.js` (93.94% match)
- `js/method-chain/inline_merge.js` (86.67% match)
- `js/method-chain/issue-4125.js` (92.81% match)
- `js/method-chain/multiple-members.js` (96.00% match)
- `js/method-chain/object-literal.js` (73.33% match)
- `js/method-chain/pr-7889.js` (33.33% match)
- `js/method-chain/print-width-120/constructor.js` (71.43% match)
- `js/new-expression/call.js` (75.00% match)
- `js/new-expression/new_expression.js` (55.56% match)
- `js/optional-chaining/chaining.js` (72.41% match)
- `js/performance/nested-real.js` (98.81% match)
- `js/preserve-line/argument-list.js` (92.66% match)
- `js/preserve-line/member-chain.js` (87.69% match)
- `js/preserve-line/parameter-list.js` (97.37% match)
- `js/require/require.js` (93.51% match)
- `js/sequence-break/break.js` (53.45% match)
- `js/sequence-expression/ignore.js` (42.86% match)
- `js/unicode/nbsp-jsx.js` (22.22% match)
- `js/yield/jsx-without-parenthesis.js` (50.00% match)
- `js/yield/jsx.js` (50.00% match)

## TypeScript Tests Failing in Dev Branch (329 total)

The TypeScript tests showing failures include all the JSX tests (which are shared) plus TypeScript-specific tests:

### TypeScript-Specific Failing Tests
- `typescript/ambient/ambient.ts` (88.24% match)
- `typescript/angular-component-examples/15934-computed.component.ts` (76.92% match)
- `typescript/angular-component-examples/15934.component.ts` (53.85% match)
- `typescript/angular-component-examples/test.component.ts` (41.18% match)
- `typescript/argument-expansion/argument_expansion.ts` (93.22% match)
- `typescript/argument-expansion/arrow-with-return-type.ts` (89.47% match)
- `typescript/array/comment.ts` (87.50% match)
- `typescript/arrow/16067.ts` (90.91% match)
- `typescript/arrow/comments.ts` (44.44% match)
- `typescript/as/as.ts` (67.72% match)
- `typescript/as/assignment.ts` (86.67% match)
- `typescript/as/assignment2.ts` (94.12% match)
- `typescript/as/export_default_as.ts` (0.00% match)
- `typescript/as/expression-statement.ts` (75.00% match)
- `typescript/as/long-identifiers.ts` (81.48% match)
- `typescript/as/nested-await-and-as.ts` (16.67% match)
- `typescript/as/ternary.ts` (80.00% match)
- `typescript/assert/comment.ts` (0.00% match)
- `typescript/assert/index.ts` (75.00% match)

And many more TypeScript-specific tests across categories including:
- Assignment expressions
- Call signatures
- Cast expressions
- Chain expressions
- Classes
- Comments
- Compiler tests
- Conditional types
- Conformance tests
- Decorators
- Enum declarations
- Export/import statements
- Function types
- Generic types
- Interface declarations
- Intersection types
- Mapped types
- Method signatures
- Modules
- Optional types
- Satisfies operators
- Template literal types
- Trailing commas
- Tuple types
- Type aliases
- Type arguments
- Union types
- And more

## Key Observations

1. **Comment handling** appears to be a major area of regression, with many comment-related tests failing
2. **JSX formatting** is heavily affected, with most JSX tests failing
3. **Ternary expressions** and conditional formatting show significant regressions
4. **Template literals** have widespread formatting issues
5. **Semicolon handling** (no-semi mode) shows failures
6. **Object and array formatting** has multiple regressions
7. **TypeScript-specific syntax** (generics, type assertions, interfaces) shows failures

The high match percentages (many above 70-90%) suggest these are often minor formatting differences rather than complete failures, but they still need to be addressed to restore compatibility.

# Action Plan to Address Remaining Tests

Based on the analysis above and following the implementation guidelines (prefer simplicity, leverage existing utilities, avoid loops), here is the proposed action plan ordered from simplest to most complex:

## Priority 1: High Match Percentage / Simple Fixes (80%+ match)

These tests have very high match percentages suggesting minor formatting differences that should be relatively easy to fix:

1. **Template and String Handling (Simple Cases)**
   - `js/template-literals/expressions.js` (88.89% match)
   - `js/strings/escaped.js` (73.68% match)
   - `js/template/indent.js` (85.71% match)

2. **High-Match Function/Method Calls**
   - `js/call/first-argument-expansion/test.js` (98.76% match)  
   - `js/performance/nested-real.js` (98.81% match)
   - `js/method-chain/comment.js` (97.56% match)
   - `js/preserve-line/parameter-list.js` (97.37% match)
   - `js/comments/export.js` (97.37% match)

3. **TypeScript High-Match Cases**
   - `typescript/arrow/16067.ts` (90.91% match)
   - `typescript/argument-expansion/argument_expansion.ts` (93.22% match)
   - `typescript/as/assignment2.ts` (94.12% match)

## Priority 2: Comment Handling Fixes (Medium Complexity)

Since comment handling appears to be a major regression area, investigate and fix:

4. **Comment Placement Logic**
   - Review formatter's comment attachment logic 
   - Focus on high-match comment tests first (90%+ match)
   - `js/comments/blank.js` (95.24% match)
   - `js/comments/empty-statements.js` (90.91% match)
   - `js/comments/function-declaration.js` (91.06% match)

## Priority 3: Semicolon and Statement Formatting

5. **No-Semi Mode Issues**
   - `js/no-semi/no-semi.js` (90.66% match)
   - `js/no-semi/class.js` (46.55% match)
   - Investigation: Check if semicolon insertion logic changed

## Priority 4: Object and Property Formatting

6. **Object Formatting**
   - `js/object-prop-break-in/test.js` (88.57% match)
   - `js/object-property-comment/after-key.js` (85.71% match)
   - Focus on property layout and spacing

## Priority 5: Arrow Functions and Function Calls

7. **Arrow Function Formatting**
   - `js/arrows/call.js` (91.01% match) 
   - `js/break-calls/break.js` (87.23% match)
   - Check parentheses placement around function arguments

## Priority 6: JSX and React Formatting (Higher Complexity)

8. **JSX Comment Handling** 
   - Start with higher-match JSX tests
   - `jsx/fragment/fragment.js` (78.57% match)
   - `jsx/binary-expressions/relational-operators.js` (78.95% match)

9. **JSX Structure and Spacing**
   - Focus on JSX element structure preservation
   - JSX attribute formatting

## Priority 7: Complex Expression Formatting

10. **Ternary and Conditional Expressions**
    - These have lower match percentages, suggesting more complex issues
    - Start with investigation into conditional expression parentheses
    - Check if ternary indentation logic changed

11. **Template Literals (Complex Cases)**
    - `js/template-literals/binary-exporessions.js` (0.00% match)
    - `js/template-literals/conditional-expressions.js` (0.00% match)
    - These likely need more significant logic changes

## Priority 8: TypeScript-Specific Syntax

12. **TypeScript Type Annotations**
    - Generic type parameter formatting
    - Interface and type alias formatting
    - Cast expression parentheses

## Implementation Strategy

For each priority group:

1. **Debug Investigation**: Use the conformance test tool with `--filter` to examine specific failing cases:
   ```bash
   cargo run -p oxc_prettier_conformance -- --filter <test-name>
   ```

2. **Root Cause Analysis**: Identify what changed in the formatter that's causing these specific issues. Since the problem stems from removing `AstKind::Argument`, look for:
   - Places where argument detection was used for formatting decisions
   - Missing parentheses around function arguments
   - Changed spacing/indentation around function calls

3. **Targeted Fixes**: Implement minimal compensating logic rather than trying to restore removed infrastructure

4. **Validation**: After each fix, run the full conformance test suite to ensure no regressions

The key insight is that many of these failures have high match percentages, suggesting the core formatting logic is working but specific edge cases around argument/parameter handling need targeted fixes.

# Implementation Progress Log

## Fix #1: Object Literal Parentheses (COMPLETED)
**Date**: Completed in current session
**Files Modified**: 
- `crates/oxc_formatter/src/parentheses/expression.rs`

**What was done**:
- Added check in `NeedsParentheses` for `ObjectExpression` to detect if the object is a function argument
- Used existing utility function `is_expression_used_as_call_argument` from `crates/oxc_formatter/src/utils/mod.rs`
- Simple 4-line addition that checks if object expression is used as argument before applying parentheses rules

**Results**:
- JavaScript: +26 tests fixed (464 → 490)
- TypeScript: +2 tests fixed (244 → 246)
- No regressions detected
- Total improvement: 28 tests now passing

**Key insight**: The utility function `is_expression_used_as_call_argument` already existed in the codebase but wasn't being used for object expressions. This demonstrates the importance of checking for existing utilities before implementing new ones.

## Previous Investigation Results

### Priority 2: Comment Handling (ANALYZED - Complex Fix Required)
**Status**: Investigated but not implemented - requires significant refactoring

**Analysis Summary**:
The comment placement issue in method chains (e.g., `js/method-chain/comment.js` at 97.56% match) is more complex than initially anticipated:

1. **The Problem**: Comments placed inline within method chains are being moved to the end of the entire chain
   - Example: `.consolidated // comment` → `.consolidated.merge()` becomes `.consolidated.merge() // comment`

2. **Root Cause**: 
   - Comments are attached to AST nodes (like StaticMemberExpression) as trailing comments
   - The current formatting logic processes these as end-of-line comments
   - There's no mechanism to preserve inline comment positions within chains

3. **Why It's Complex**:
   - Would require modifying how comments are attached to AST nodes
   - Need to distinguish between "inline trailing" vs "end-of-line trailing" comments
   - The comment system is generated code that would need careful modification
   - Risk of breaking other comment formatting that currently works

4. **Recommendation**: 
   - This fix requires architectural changes to the comment system
   - Should be deferred to a separate PR focused specifically on comment handling
   - The current 26-test improvement from the object literal fix is substantial progress

5. **Other Comment Issues Observed**:
   - Export statement spacing after comments (`export //comment` missing space before `{}`)
   - Template literal comment migration (comments inside `${}` moving outside)
   - Parameter list preserve-line issues (extra blank lines between parameters)

These issues all stem from the fundamental comment attachment mechanism and would benefit from a unified solution rather than individual patches.

## Priority 3: Semicolon and No-Semi Mode Investigation (ANALYZED - Partially Complex)
**Date**: Investigated in current session
**Status**: Not implemented - requires careful handling

**Analysis Summary**:
The no-semi mode issues involve two separate problems:

1. **Missing comma in spread/rest syntax** (e.g., `[a, b, ...c]` → `[a, b ...c]`)
   - Root cause: `write_array_node` doesn't know about rest elements following, so doesn't add comma after last element
   - Attempted fix caused double comma issues with trailing commas
   - Would require refactoring how array elements and rest are formatted together
   
2. **Leading semicolons being removed** (e.g., `;[1,2,3]` → `[1,2,3]`)
   - These semicolons are necessary in no-semi mode to prevent ASI issues
   - Example: Without semicolon, `x[1,2,3]` would be parsed as property access instead of two statements
   - Would require ASI-aware formatting logic

**Complexity Assessment**:
- The spread/rest comma issue requires coordinating between `write_array_node` and rest element formatting
- The leading semicolon issue requires understanding ASI rules and when statements need protection
- Both issues are more complex than the object literal parentheses fix

**Recommendation**:
- These issues should be addressed separately with careful design
- The spread/rest issue affects code correctness and should be prioritized
- The leading semicolon issue is a style preference that doesn't break code

# Detailed Implementation Plan for Priority 1 Fixes

## Background Context
The removal of `AstKind::Argument` from the AST has disrupted the formatter's ability to correctly identify when an expression is a function argument. This affects multiple formatting decisions, particularly around parentheses placement. The high match percentages (88-98%) indicate these are mostly minor formatting differences rather than fundamental logic issues.

**Important**: Before starting implementation, review the git history to understand how `AstKind::Argument` was previously used:
```bash
git log -p --grep="Argument" -- crates/oxc_formatter
git diff <commit-before-removal> <commit-after-removal> -- crates/oxc_formatter
```

## Identified Issues and Root Causes

### Issue 1: Object Literal Parentheses (Critical - Affects Many Tests)
**What's happening**: Object literals `{}` are being unnecessarily wrapped in parentheses when they appear as function arguments.

**Examples of the problem**:
```javascript
// Expected (Prettier output):
RecordImport.find({}, "", {}, callback)

// Current (Our broken output):
RecordImport.find(({}), "", ({}), callback)
```

**Root Cause**: The formatter can no longer determine that an object expression is a function argument and is applying default parentheses rules for object literals.

**Files to examine**:
- `crates/oxc_formatter/src/parentheses/expression.rs` - Contains `NeedsParentheses` trait implementations
- Look for `impl<'a> NeedsParentheses<'a> for AstNode<'a, ObjectExpression<'a>>`

### Issue 2: Comment Placement in Method Chains
**What's happening**: Comments are being moved from their intended inline positions to the end of the chain.

**Example of the problem**:
```javascript
// Expected (Prettier output):
  .consolidated // global/default values (do NOT modify)
  .merge(this.cachedWorkspaceConfig);

// Current (Our broken output):
  .consolidated.merge(this.cachedWorkspaceConfig); // global/default values (do NOT modify)
```

**Root Cause**: Without argument context, comment attachment logic is defaulting to end-of-line placement instead of preserving inline positions.

**Files to examine**:
- `crates/oxc_formatter/src/formatter/comments/mod.rs` - Comment attachment logic
- `crates/oxc_formatter/src/write/member_chain/` - Method chain formatting
- Check for existing comment utilities in `crates/oxc_formatter/src/formatter/trivia/`

### Issue 3: Parameter List Extra Line Breaks
**What's happening**: Extra blank lines are being inserted between function parameters when formatted multi-line.

**Example of the problem**:
```javascript
// Expected (Prettier output):
function(done, foo) {

// Current (Our broken output):  
function(
  done,
  
  foo,
) {
```

**Root Cause**: Parameter list formatting is missing context about being inside a parameter list, applying generic line break rules.

**Files to examine**:
- `crates/oxc_formatter/src/write/parameter_list.rs` - Parameter formatting logic
- `crates/oxc_formatter/src/write/call_arguments.rs` - Argument list formatting

### Issue 4: Template Literal Expression Formatting
**What's happening**: Multiple issues with template literals:
- Comments inside template expressions being moved outside
- Conditional expressions getting unnecessary parentheses
- Expressions being broken unnecessarily

**Example of the problem**:
```javascript
// Expected (Prettier output):
`text ${a.b //comment
} more text`

// Current (Our broken output):
`text ${a.b} more text` //comment

// Another example - unnecessary parentheses:
// Expected:
`text ${condition ? "yes" : "no"} text`
// Current:
`text ${(condition ? "yes" : "no")} text`
```

**Root Cause**: Template expression formatting is losing context about being inside template placeholders.

**Files to examine**:
- Search for `TemplateLiteral` and `TemplateElement` in `crates/oxc_formatter/src/`
- Check `crates/oxc_formatter/src/generated/format.rs` for template literal formatting

### Issue 5: TypeScript Index Signature Separators
**What's happening**: Semicolons being incorrectly added in various contexts where commas should be used.

**Example of the problem**:
```typescript
// Expected (Prettier output):
{ [key: number]: boolean }

// Current (Our broken output):
{ [key: number]: boolean; }
```

**Root Cause**: Type member separator logic not distinguishing between:
- Interface members (use semicolon or comma)
- Object type literals inline in type arguments (use comma)
- Type parameters in generics (use comma)

**Files to examine**:
- Search for `TSTypeLiteral` and `TSIndexSignature` in `crates/oxc_formatter/src/`
- Check `crates/oxc_formatter/src/generated/format.rs` for TypeScript type formatting

## Step-by-Step Implementation Guide

### Step 1: Use Existing Argument Detection Utility
**Goal**: Use the existing helper function to determine if an expression is a function argument.

**Available Utility**:
```rust
// Already exists in crates/oxc_formatter/src/utils/mod.rs
pub fn is_expression_used_as_call_argument(span: Span, parent: &AstNodes) -> bool
```

This utility was already in the codebase and is what made the successful object literal fix possible. It checks if an expression's span matches any argument spans in the parent CallExpression or NewExpression.

### Step 2: Fix Object Literal Parentheses
**Goal**: Prevent unnecessary parentheses around object literals in function arguments.

**Implementation**:
1. Locate `impl<'a> NeedsParentheses<'a> for AstNode<'a, ObjectExpression<'a>>` in `crates/oxc_formatter/src/parentheses/expression.rs`
2. Modify the `needs_parentheses` method:
```rust
fn needs_parentheses(&self, f: &Formatter<'_, 'a>) -> bool {
    let parent = self.parent;
    
    // Add this check FIRST - note we pass parent and self.span
    if is_function_argument(parent, self.span) {
        return false; // No parentheses needed for function arguments
    }
    
    // Keep existing logic for other cases
    is_class_extends(parent, self.span())
        || is_first_in_statement(
            self.span,
            parent,
            FirstInStatementMode::ExpressionStatementOrArrow,
        )
}
```

### Step 3: Fix Comment Attachment in Method Chains
**Goal**: Preserve inline comment positions in method chains.

**Implementation**:
1. Find the method chain formatting logic (likely in `crates/oxc_formatter/src/write/member_chain/`)
2. Check for comments in the trivia between chain members
3. Preserve the comment's attachment point (leading vs trailing)
4. Specific checks needed:
   - Look for trailing comments after a member expression
   - Check if comment should stay on same line or break to next line
   - Use existing utilities from `crates/oxc_formatter/src/formatter/trivia/`

### Step 4: Fix Parameter List Line Breaks
**Goal**: Remove extra blank lines between parameters.

**Implementation**:
1. In `crates/oxc_formatter/src/write/parameter_list.rs`
2. Find where line breaks are added between parameters
3. Ensure only single line breaks are added, not double

### Step 5: Fix Template Literal Issues
**Goal**: Preserve comment positions and avoid unnecessary parentheses in template expressions.

**Implementation**:
1. Find template literal formatting (search for `TemplateLiteral` in formatter)
2. For expressions inside `${}`:
   - Preserve comments that are inside the expression
   - Check if parentheses are actually needed (most expressions don't need them)
   - Use the `is_function_argument` utility to make better decisions

### Step 6: Fix TypeScript Separator Issues
**Goal**: Use correct separators in TypeScript type definitions.

**Implementation**:
1. Find TypeScript object type formatting
2. Ensure index signatures in object type literals use commas, not semicolons
3. Check parent context to determine correct separator

## Testing Each Fix

After implementing each step:

1. **Test the specific failing case**:
```bash
cargo run -p oxc_prettier_conformance -- --filter "test-name"
```

2. **Run the full conformance suite** to ensure no regressions:
```bash
cargo run -p oxc_prettier_conformance
```

3. **Document the improvement** in passing test count.

## Expected Outcomes

After completing all Priority 1 fixes (conservative estimates):
- Object literal parentheses fix: Should resolve ~5-8 tests
- Comment placement fix: Should resolve ~2-3 tests  
- Parameter formatting fix: Should resolve ~1-2 tests
- Template literal fixes: Should resolve ~2-3 tests
- TypeScript fixes: Should resolve ~1-2 tests

**Total expected improvement**: 10-15 of the 41 failing JavaScript tests should pass.

**Note**: These are conservative estimates. Some fixes may have cascading positive effects that resolve additional tests. However, there's also risk of regression where a fix might break currently passing tests.

## Important Notes for Implementers

1. **DO NOT** try to recreate the `AstKind::Argument` infrastructure - use targeted fixes only
2. **Test incrementally** - each fix should be tested independently before moving to the next
3. **Watch for regressions** - some fixes might break other tests, so run full suite after each change
4. **Use existing utilities** - leverage existing formatter utilities where possible
5. **Keep changes minimal** - the smaller the change, the easier to debug if something goes wrong
6. **Have a rollback strategy** - Keep each fix in a separate commit so you can revert if a fix causes more harm than good
7. **Check the Argument enum structure** - Remember that `Argument` has variants (Expression, SpreadElement) that need proper matching

## Debugging and Troubleshooting

### Common Issues and Solutions

| Problem | Solution |
|---------|----------|
| **"Found 2 matches" error when editing** | Provide more context around the code to make it unique |
| **Double comma in arrays** | Check if `write_array_node` is already adding commas |
| **Tests pass individually but fail together** | One test may be exiting early, hiding other failures |
| **Can't find where node is formatted** | Check `crates/oxc_formatter/src/generated/format.rs` |
| **Parentheses added unexpectedly** | Check `NeedsParentheses` implementations in `parentheses/` |

### Debugging Commands

```bash
# Test a specific fix
cargo run -p oxc_prettier_conformance -- --filter "js/call/first-argument-expansion/test.js"

# See the actual vs expected diff
cargo test -p oxc_prettier_conformance 2>&1 | grep -B5 -A15 "💥"

# Run full suite to check for regressions
just coverage

# Check test count improvement
cargo run -p oxc_prettier_conformance | grep "compatibility:"

# Debug with print statements (add to formatter code)
eprintln!("Parent type: {:?}", parent);
eprintln!("Node span: {:?}", self.span());
```

### Understanding Test Output

- **💥** = Test is failing
- **✨** = Test would pass with different options
- **Match %** = How much of the output matches Prettier (higher is better)
- Files with 0% match usually have fundamental issues (missing elements, wrong structure)
- Files with 90%+ match usually have minor issues (spacing, parentheses)

## Investigation Insights and Lessons Learned

### Key Patterns in Formatter Issues

1. **Parentheses Issues (Most Common)**
   - Root cause: Loss of `AstKind::Argument` context
   - Solution pattern: Use `is_expression_used_as_call_argument` utility
   - Affected nodes: ObjectExpression, ClassExpression, potentially others
   - Success rate: High - simple targeted fixes work well

2. **ASI (Automatic Semicolon Insertion) Issues**
   - Examples: Arrow functions, no-semi mode
   - Root cause: Missing ASI-aware logic in formatter
   - Complexity: HIGH - requires understanding JavaScript ASI rules
   - Recommendation: Defer until dedicated ASI implementation can be added

3. **Line Breaking for Long Arguments**
   - Examples: ImportExpression, long strings in calls
   - Root cause: Simple formatters don't handle line breaking
   - Complexity: HIGH - requires CallArguments-like logic
   - Key insight: `group` and `soft_block_indent` need careful type handling

4. **Comment Positioning**
   - Root cause: Comment attachment happens at a different phase
   - Complexity: Variable - dangling comments easier than inline
   - Key insight: Comments need parent context that may not be available

### Common Pitfalls to Avoid

1. **Don't assume low complexity based on test name**
   - "trailing-comma/dynamic-import.js" wasn't about trailing commas at all
   - Always check actual test output to understand the real issue

2. **Type system constraints in formatter**
   - `Format` trait has specific requirements
   - `format_args!` macro doesn't always work with formatter functions
   - Look for existing patterns using `format_once` or similar

3. **Cascading effects of fixes**
   - Array rest element fix caused double comma issues
   - Always run full test suite after changes
   - Some fixes may break other tests

### Successful Fix Patterns

1. **Parentheses Context Check** (High Success Rate)
   ```rust
   // Check if expression is used as function argument
   if is_expression_used_as_call_argument(self.span, parent) {
       return false;  // Don't add parentheses
   }
   ```

2. **Look for Existing Utilities**
   - `is_expression_used_as_call_argument` - checks if node is a call argument
   - `is_first_in_statement` - checks if expression starts a statement
   - `format_dangling_comments` - handles comments in empty structures

### Areas Requiring Architectural Changes

1. **ASI Implementation**
   - Needed for: Arrow semicolons, no-semi mode, leading semicolons
   - Requires: Full ASI rule implementation
   - Impact: Would fix multiple test categories

2. **Unified Argument Formatting**
   - Needed for: ImportExpression, potentially others
   - Current state: CallExpression has complex logic not shared
   - Proposal: Extract shared argument formatting utility

3. **Comment System Refactoring**
   - Needed for: Method chain comments, complex inline comments
   - Current limitation: Comments lack parent context at format time
   - Impact: Would fix numerous comment-related tests

## Final Summary

### Achievements
- Successfully recovered 28 tests (26 JS + 2 TS) by fixing object literal parentheses issue
- Current test status: JS: 490/699 (70.10%), TS: 246/573 (42.93%)
- Improvement from dev branch baseline: +26 JS tests, +2 TS tests

### Key Learnings
1. **Targeted fixes are most effective:** The object literal fix was successful because it addressed a specific, well-defined issue
2. **Complex issues require architectural changes:** Comment handling and array formatting issues revealed deeper architectural limitations
3. **Existing utilities are valuable:** The `is_expression_used_as_call_argument` utility was key to the successful fix

### Remaining Work
The remaining 15 JS and 3 TS test failures are primarily related to:
- Complex comment placement in method chains and conditional expressions
- Array/destructuring formatting with rest elements
- Semicolon insertion/preservation in no-semi mode
- Template literal and JSX expression formatting

These issues generally require more extensive refactoring and careful consideration of the formatter's architecture rather than simple targeted fixes.

## Detailed Breakdown of Remaining Issues

### Priority 2: Comment Handling - BREAKDOWN

Breaking down comment handling into smaller, manageable sub-tasks:

#### Priority 2.1: Dangling Comments in Empty Arrays ✅ INVESTIGATED - Not Actually Broken
- **Tests:** `js/comments/dangling_array.js` (40% match)
- **Initial Assessment:** Comments in empty arrays not positioned correctly
- **Investigation Results:** 
  - The 40% match rate is MISLEADING - it's due to ASI (leading semicolon) issues, not comment handling
  - Dangling comments in arrays are actually working correctly
  - Test case `[/* dangling */]` formats properly
  - Test case `[// Nothing.]` formats properly
  - The only difference is missing leading semicolon: `;[1, 2, 3]` vs `[1, 2, 3]`
- **Actual Issue:** This is an ASI problem, not a comment problem
- **Status:** NO FIX NEEDED - dangling array comments work correctly
- **Recommendation:** The low match percentage is due to ASI issues already documented in Priority 3.2

#### Priority 2.2: Inline Comments in Arrays/Objects ⭐⭐ (Medium Complexity)
- **Tests:** Multiple array/object comment tests
- **Issue:** Comments between elements not preserving position
- **Example:** `[1 /* first */, 2 /* second */]`
- **Complexity:** Medium - requires tracking comment position relative to elements
- **Approach:** Review comment attachment in `write_array_node`

#### Priority 2.3: Method Chain Comments ⭐⭐⭐ (High Complexity)
- **Tests:** `js/method-chain/comment.js` (97.56% match - nearly passing!)
- **Issue:** Comments in method chains affecting line breaking
- **Complexity:** High - requires architectural changes
- **Note:** Already investigated - needs major refactoring

#### Priority 2.4: JSX Comments ⭐⭐ (Medium Complexity)
- **Tests:** Various JSX comment tests (0-42% match)
- **Issue:** Comments in JSX expressions and attributes
- **Complexity:** Medium - JSX has special comment rules
- **Approach:** Review JSX-specific comment handling

#### Priority 2.5: Closure Type Cast Comments ⭐⭐ (Medium Complexity)
- **Tests:** `js/comments-closure-typecast/*` (0-90% match)
- **Issue:** Special comment syntax for closure compiler `/** @type {Type} */ (expr)`
- **Complexity:** Medium - special parsing/formatting rules
- **Approach:** May need special case handling for these comment patterns

### Priority 3: Semicolon and Array Issues - BREAKDOWN

Breaking down into more specific sub-tasks:

#### Priority 3.1: Arrow Function Semicolons ⭐ (Low Complexity)
- **Tests:** `js/arrows/semi/semi.js` (0% match)
- **Issue:** Missing semicolon after arrow function expressions
- **Example:** `a => {}` should potentially have semicolon
- **Complexity:** Low - statement termination detection
- **Approach:** Check arrow function statement detection

#### Priority 3.2: No-Semi Mode ASI Rules ⭐⭐⭐ (High Complexity)
- **Tests:** `js/no-semi/*` (36-90% match)
- **Issue:** Leading semicolons for ASI disambiguation
- **Example:** `x; [1,2]` needs semicolon to prevent `x[1,2]`
- **Complexity:** High - requires full ASI implementation
- **Note:** Already investigated - needs ASI-aware logic

#### Priority 3.3: Array Rest Element Commas ⭐⭐ (Medium Complexity)
- **Tests:** `js/rest/trailing-commas.js` and related
- **Issue:** Missing comma before rest in `[a, b, ...rest]`
- **Complexity:** Medium - array formatting coordination
- **Note:** Already attempted - needs careful redesign to avoid double commas

#### Priority 3.4: Array Spread in Expressions ✅ INVESTIGATED - JS Works, JSX Not Implemented
- **Tests:** `js/array-spread/*`, `js/spread/*`, `jsx/spread/*`
- **Initial Assessment:** Spread element formatting in various contexts
- **Investigation Results:**
  - JavaScript spread/rest syntax is working correctly
  - All JS spread tests pass: `js/spread/spread.js`, `js/array-spread/multiple.js`, `js/babel-plugins/object-rest-spread.js`
  - The failing "spread" tests are actually JSX-related: `jsx/spread/attribute.js`, `jsx/spread/child.js`
  - JSX formatting appears to NOT be implemented in the formatter at all
  - No JSX-related code found in `crates/oxc_formatter/src/`
- **Actual Issue:** JSX support is missing from the formatter, not a spread syntax problem
- **Status:** NO FIX POSSIBLE - requires JSX formatter implementation
- **Recommendation:** JSX support would be a major feature addition, not a simple fix

#### Priority 3.5: Trailing Commas in Dynamic Imports ⭐⭐⭐ (Higher Complexity Than Expected)
- **Tests:** `js/trailing-comma/dynamic-import.js` (0% match)
- **Issue:** Not actually about trailing commas - it's about line breaking for long module names
- **Example:** 
  ```javascript
  // Input:
  import('verylongmodulename...')
  
  // Expected (Prettier):
  import(
    "verylongmodulename..."
  );
  
  // Current (Our output):
  import("verylongmodulename...");
  ```
- **Complexity:** HIGH - Initially seemed simple but requires significant refactoring
- **Investigation Results:** 
  - The `ImportExpression` formatter in `crates/oxc_formatter/src/write/import_declaration.rs` is too simplistic
  - Currently just writes `import(source)` without any line breaking logic
  - Needs to behave like `CallExpression` with proper argument formatting
  - Would require implementing `CallArguments`-like logic for import expressions
  - The `group` and `soft_block_indent` patterns used elsewhere need careful adaptation
- **Attempted Fix:** Tried to add grouping and indentation but ran into type system issues with the formatter's `Format` trait
- **Recommendation:** DEFER - This requires architectural changes similar to how `CallExpression` handles arguments. Consider implementing a shared argument formatting utility that both `CallExpression` and `ImportExpression` can use.

### Priority 4: Other Formatting Issues - NEW

#### Priority 4.1: Class Expression Parentheses ✅ COMPLETED
- **Tests:** `js/classes/call.js` 
- **Issue:** Class expressions as arguments needed proper parentheses
- **Solution:** Applied same pattern as object literal fix
- **Status:** FIXED - 3 JS tests recovered

#### Priority 4.2: Template Literal Expressions ⭐⭐ (Medium Complexity)
- **Tests:** Various template literal tests (0-88% match)
- **Issue:** Expression formatting within template literals
- **Complexity:** Medium - nested expression handling

### Recommended Implementation Order

Start with the simplest fixes first to build momentum and understanding. **Remember to commit each fix separately!**

#### 🟢 Start Here (Low Complexity - Similar to Completed Fix)

**NEXT ACTION:** 
1. Start with **Priority 1.1** - Fix new expression calls (`js/new-expression/call.js`)
   - Similar to the class expression fix we already did
   - Check `NeedsParentheses` for `NewExpression`
   - Pattern: Detect when new expression is the callee of a call

**COMPLETED:**
- ✅ Priority 1 - Object Literal Parentheses (26 JS + 2 TS tests fixed)
- ✅ Priority 4.1 - Class Expression Parentheses (3 JS tests fixed)

**INVESTIGATED - NO FIX NEEDED:**
- ✅ Priority 2.1 - Dangling Comments in Empty Arrays (already working correctly, low match % is due to ASI)
- ✅ Priority 3.4 - Array Spread in Expressions (JS works correctly, failures are JSX-only)

**INVESTIGATED BUT TOO COMPLEX:**
- ❌ Priority 3.1 - Arrow Function Semicolons (needs ASI implementation)
- ❌ Priority 3.5 - Dynamic Import Line Breaking (needs CallArguments-like logic)
- ❌ JSX Support - Not implemented at all, would be major feature addition

#### 🟡 Medium Complexity (Requires More Understanding)
3. **Priority 2.2** - Inline Comments in Arrays/Objects  
4. **Priority 3.3** - Array Rest Element Commas (attempted but needs redesign)
5. **Priority 2.4** - JSX Comments
6. **Priority 2.5** - Closure Type Cast Comments
7. **Priority 4.2** - Template Literal Expressions

#### 🔴 High Complexity (Architectural Changes Needed)
8. **Priority 2.3** - Method Chain Comments (investigated - needs refactoring)
9. **Priority 3.2** - No-Semi Mode ASI Rules (investigated - needs ASI implementation)