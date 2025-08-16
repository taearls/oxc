# Formatter Conformance Fix Guide

## ⚠️ CRITICAL CONTEXT: AstKind::Argument Removal

**This development branch contains a major refactoring that removed `AstKind::Argument` from the AST.**

The branch `temp/resume-fixing-conformance-tests` has 150+ commits on top of main that refactor how arguments are handled throughout the formatter. This refactoring is the **root cause** of all regressions.

## Current Status (UPDATED)

| Branch                    | JavaScript           | TypeScript           | Notes                      | Commit      |
| ------------------------- | -------------------- | -------------------- | -------------------------- | ----------- |
| **Main**                  | 525/699 (75.11%)     | 277/573 (48.34%)     | Baseline                   | N/A         |
| **Development (Initial)** | 513/699 (73.39%)     | 274/573 (47.82%)     | Before any fixes           | `ca46ab9f7` |
| **After Decorator Fix**   | 514/699 (73.53%)     | 274/573 (47.82%)     | +1 JS test                 | `1a9e0a154` |
| **After Angular Fix**     | **517/699 (73.96%)** | **274/573 (47.82%)** | +3 JS tests                | `883108d18` |
| **Remaining to Fix**      | **+8 JS tests**      | **+3 TS tests**      | To reach main branch level | -           |

## ✅ COMPLETED FIXES

### 1. Decorator Regression (FIXED)

**Fixed in commit**: `1a9e0a154 fix conformance test`

**Problem**: Decorated class expressions in extends clauses were missing parentheses.

**Solution Applied**:

```rust
// In crates/oxc_formatter/src/parentheses/expression.rs
// Added to ClassExpression NeedsParentheses impl:
if !self.decorators.is_empty() && is_class_extends(parent, self.span) {
    return true;
}

// Helper function added:
fn is_class_extends(parent: &AstNodes, span: Span) -> bool {
    matches!(parent, AstNodes::Class(class) if class.super_class.as_ref()
        .map_or(false, |super_class| super_class.span() == span))
}
```

**Tests Fixed**:

- ✅ `js/decorators/class-expression/super-class.js` (14.29% → 100%)

### 2. Arrow Function Currying (FIXED)

**Fixed in development branch**

**Problem**: Deeply curried arrow functions (3+ levels) weren't breaking lines correctly.

**Key Discovery**: Arrow functions use **Chain layout**, not Single layout for currying!

**Solution Applied**:

```rust
// In crates/oxc_formatter/src/write/arrow_function_expression.rs

// Added helper to count currying depth:
fn count_currying_depth(expr: &ArrowFunctionExpression) -> usize {
    let mut depth = 1;
    let mut current = &expr.body;
    while let Statement::ExpressionStatement(stmt) = current {
        if let Expression::ArrowFunctionExpression(arrow) = &stmt.expression {
            depth += 1;
            current = &arrow.body;
        } else {
            break;
        }
    }
    depth
}

// Modified Chain layout implementation:
impl Format for Chain {
    fn format(&self, f: &mut Formatter) -> FormatResult {
        // ...
        let is_deeply_curried = if let Expression::ArrowFunctionExpression(arrow) = &expr {
            count_currying_depth(arrow) >= 3
        } else {
            false
        };
        
        if is_deeply_curried {
            write!(f, [if_break(indent(line()), space())])?;
        }
        // ...
    }
}
```

**Tests Improved**:

- ✅ `js/arrows/currying-*.js` tests showing improvement

### 3. Angular Test Wrappers (FIXED)

**Fixed in commit**: `883108d18 fix(formatter): keep Angular test wrapper arguments inline`

**Problem**: Angular test wrappers (`async()`, `fakeAsync()`, `waitForAsync()`, `inject()`) were breaking their arrow function arguments onto multiple lines.

**Solution Applied**:

```rust
// In crates/oxc_formatter/src/write/call_arguments.rs
// Added detection for Angular wrappers:
let is_angular = matches!(&call.callee,
    Expression::Identifier(ident) if
    matches!(ident.name.as_str(), "async" | "inject" | "fakeAsync" | "waitForAsync")
);

// Keep their arrow arguments inline:
if is_angular_wrapper && self.len() == 1 && self.first().is_some_and(|arg| 
    matches!(arg.as_ref(), Argument::ArrowFunctionExpression(_))
) {
    return write!(
        f,
        [
            l_paren_token,
            format_once(|f| self.first().unwrap().fmt(f)),
            r_paren_token
        ]
    );
}
```

**Tests Fixed**:

- ✅ `js/test-declarations/angular_async.js` (86.21% → 100%)
- ✅ `js/test-declarations/angular_fakeAsync.js` (75.86% → 100%)
- ✅ `js/test-declarations/angular_waitForAsync.js` (75.86% → 100%)

## ❌ PENDING FIXES - PRIORITIZED BY IMPACT

### Priority Analysis (8 JS + 3 TS tests needed)

**JavaScript Categories by Impact:**

1. **js/ternaries** (9 tests, avg 31.2% match) - HIGH IMPACT
2. **js/test-declarations** (4 tests, avg 65.7% match) - MEDIUM IMPACT
3. **js/comments** (22 tests, avg 68.6% match) - MEDIUM IMPACT
4. **js/arrows** (4 tests, avg 69.8% match) - LOW IMPACT

**TypeScript Categories by Impact:**

1. **typescript/satisfies-operators** (13 tests) - HIGH IMPACT
2. **typescript/conditional-types** (6 tests) - MEDIUM IMPACT
3. **typescript/trailing-comma** (4 tests) - LOW IMPACT

### Investigation Plan

#### 1. Ternary Operators (HIGHEST PRIORITY)

**Files to investigate**: `crates/oxc_formatter/src/format/conditional_expression.rs`

**Failing tests with worst performance**:

- `js/ternaries/indent.js` (6.2% match)
- `js/ternaries/await-expression.js` (14.3% match)
- `js/ternaries/nested-in-condition.js` (23.9% match)

**Likely issue**: Parentheses and indentation handling in nested ternaries after AstKind::Argument removal.

#### 2. Test Declarations (Nearly Fixed!)

**Files to investigate**: `crates/oxc_formatter/src/write/call_arguments.rs`

**Remaining tests**:

- `js/test-declarations/angularjs_inject.js` (91.53% match - VERY CLOSE!)
- `js/test-declarations/jest-each.js` (67.7% match)
- `js/test-declarations/jest-each-template-string.js` (27.8% match)

**Next step**: Extend Angular wrapper logic to handle `inject()` and Jest patterns.

#### 3. Comment Placement

**Files to investigate**: `crates/oxc_formatter/src/comments.rs`

**Critical failures**:

- `js/comments/comment.js` (0% match)
- `js/comments/template-literal.js` (30.4% match)
- Multiple closure typecast tests at 0% match

**Likely issue**: Comment attachment logic broken after AstKind::Argument removal.

### Quick Win Opportunities

1. **angularjs_inject.js** - At 91.53%, likely needs minor tweak to Angular wrapper detection
2. **Arrow semi tests** - Several at 0% match, likely a simple semicolon issue
3. **Binary expressions** - `js/binary-expressions/return.js` at 90% match

## Investigation Tools

### conformance_investigation_scripts.py

A comprehensive Python toolkit for analyzing conformance test regressions. Key capabilities:

**1. Test Analysis**

- Parse conformance snapshot files to extract test results
- Compare main vs development branch to identify regressions
- Group failures by category to find systematic issues
- Calculate average match percentages per category

**2. Progress Tracking**

- Track improvements after each commit with hash recording
- Generate markdown tables showing progress over time
- Identify which fixes had the most impact

**3. Prioritization**

- Find "quick wins" - tests that are >85% match and likely easy to fix
- Generate prioritized investigation plans based on impact
- Identify categories with the most failures

**4. Reporting**

- Create comprehensive analysis reports
- Generate markdown-formatted progress tables
- Provide actionable recommendations

**Usage:**

```bash
# Run full analysis
python3 conformance_investigation_scripts.py

# Or import specific functions
from conformance_investigation_scripts import (
    analyze_test_groups,
    find_quick_wins,
    track_progress_after_commit
)
```

## Testing & Verification Process

### Before Making Changes

```bash
# 1. Ensure you're on the right branch
git branch --show-current
# Should show: temp/resume-fixing-conformance-tests

# 2. Check current status
cargo run --bin oxc_prettier_conformance 2>&1 | grep "compatibility:"

# 3. Run specific failing test
cargo run --bin oxc_prettier_conformance -- --filter "test-declarations/angular"
```

### After Each Fix

```bash
# 1. Format and check code
just fmt
just check

# 2. Run tests
just test

# 3. Check conformance improvement
cargo run --bin oxc_prettier_conformance 2>&1 | grep "compatibility:"

# 4. Run specific test that was failing
cargo run --bin oxc_prettier_conformance -- --filter "your-test-name"

# 5. COMMIT ONLY SOURCE CHANGES (not snapshots!)
git add crates/oxc_formatter/src/[changed-file].rs
git commit -m "fix(formatter): [specific description of fix]"

# 6. IMPORTANT: Record progress after commit
# Save the current conformance results and commit hash for tracking
cargo run --bin oxc_prettier_conformance 2>&1 | grep "compatibility:" > progress.txt
echo "Commit: $(git rev-parse --short HEAD)" >> progress.txt
cat progress.txt

# 7. Final verification
just ready
```

## Key Files for Reference

| Component          | File                                 | Purpose                                        |
| ------------------ | ------------------------------------ | ---------------------------------------------- |
| Argument detection | `utils/mod.rs`                       | Contains `is_expression_used_as_call_argument` |
| Parentheses logic  | `parentheses/expression.rs`          | Determines when expressions need parens        |
| Arrow formatting   | `write/arrow_function_expression.rs` | Handles arrow function output                  |
| Call expressions   | `write/call_expression.rs`           | Formats function calls                         |
| Test wrappers      | Various locations                    | May need new detection logic                   |

## Understanding the Codebase

### How Formatting Works

1. **Parse** → AST is created
2. **Parentheses Check** → `NeedsParentheses` trait determines if parens needed
3. **Format** → `Format` trait outputs the actual code
4. **Layout** → Different layouts (Single, Chain, Group) handle line breaking

### Key Concepts

- **Chain Layout**: Used for curried arrows and chained calls
- **Single Layout**: Used for simple single-line expressions
- **Group Layout**: Used for grouped arguments and parameters

## Debugging Tips

### Add Strategic Debug Output

```rust
// Temporary debug in failing area
eprintln!("DEBUG: Expression type: {:?}, parent: {:?}", 
    std::any::type_name_of_val(self), 
    std::any::type_name_of_val(parent));
```

### Compare with Main Branch

```bash
# See what changed in a specific file
git diff main HEAD -- crates/oxc_formatter/src/[file].rs

# Check implementation in main
git show main:crates/oxc_formatter/src/[file].rs
```

### Test Minimal Cases

Create `test.js` with minimal failing case:

```javascript
// For Angular issue
it(
  'test',
  async(() => {
    console.log('test');
  }),
);
```

Then test with:

```bash
cargo run --bin oxc_formatter test.js
npx prettier test.js
```

## Expected Final Outcome

Target compatibility levels:

- JavaScript: 525/699 (75.11%) ✓
- TypeScript: 277/573 (48.34%) ✓

All tests that passed in main should pass in the development branch.

## Important Reminders

1. **DO NOT commit snapshot files** (`*.snap.md`)
2. **Test each fix individually** before moving to the next
3. **Use existing helper functions** when available
4. **Follow existing code patterns** in the formatter
5. **Run `just ready`** before considering any fix complete

## Getting Help

If stuck on a specific issue:

1. Check how Prettier formats it: `npx prettier [test-file]`
2. Look for similar patterns in the codebase that work
3. Use AST parser to understand structure: `cargo run -p oxc_parser --example parser test.js`
4. Check if main branch has relevant logic: `git diff main HEAD`
