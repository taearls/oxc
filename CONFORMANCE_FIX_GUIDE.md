# Formatter Conformance Fix Guide

## Current Status
- **Now:** JS 500/699 (71.53%), TS 246/573 (42.93%)
- **Target (main branch):** JS 505/699 (72.25%), TS 249/573 (43.46%)
- **Gap:** 5 JS tests, 3 TS tests
- **Fixed so far:** Object literals, class expressions, new expressions, require.js regression, Angular test wrappers

## The Problem
`AstKind::Argument` was removed from AST → formatter can't detect function arguments → wrong parentheses and formatting

### Two Types of Issues

1. **Parentheses Issues** (✅ FIXED): Determining when expressions need parentheses based on parent context
2. **Line Breaking Issues** (🚧 REMAINING): Determining when to break arguments onto multiple lines - more complex because it involves formatting context propagation

## ✅ COMPLETED FIXES

### Require.js Line Breaking Fix
**Test:** `js/require/require.js` - Now passing!
**Solution:** Added `should_use_long_curried_formatting` to prevent `path.join` with complex arguments from breaking incorrectly.

### Angular Test Wrapper Fix
**Tests:** `angular_async.js`, `angular_fakeAsync.js`, `angular_waitForAsync.js` - All passing!
**Solution:** Added `should_not_break_angular_wrapper` to keep Angular test wrappers with single function arguments on one line.

## 🎯 Next Priority Tasks

## Complete Fix Priority List

### 🟢 Easy Fixes (Similar to What We've Done)

1. ✅ **New Expression Calls** - COMPLETED
   - Test: `js/new-expression/call.js` (was 75%, now 100%)
   - Fixed: CallExpression parentheses when used as new expression callee
   - Result: +2 JS tests passing

2. ✅ **New Expression with Member Access** - COMPLETED
   - Test: `js/new-expression/new_expression.js` (was 55.56%, now 100%)
   - Fixed: MemberExpression parentheses when object contains call/tagged template
   - Result: +1 JS test passing

### 🟡 Medium Complexity (Line Breaking Issues)

3. ✅ **Require Statement Line Breaking** - COMPLETED
   - Test: `js/require/require.js` (now passing!)
   - Fixed: `path.join` with complex arguments no longer breaks incorrectly
   - Solution: Added special case in `should_use_long_curried_formatting`

4. ✅ **Test Framework Functions** - COMPLETED (3 tests fixed!)
   - Tests: `js/test-declarations/angular_*.js` - All passing!
   - Fixed: Angular wrappers no longer break their function arguments
   - Solution: Added special case detection in `should_not_break_angular_wrapper`

5. **Curried Arrow Functions**
   - Tests: `js/arrows/currying-2.js` (59%), `currying-4.js` (78%)
   - Issue: Arrow function formatting in curried patterns
   - Complexity: Context-dependent formatting

6. **Function Composition**
   - Test: `js/functional-composition/pipe-function-calls.js` (87%)
   - Issue: Formatting of composed function calls
   - Complexity: Pattern recognition

### 🔴 Complex (Architecture Changes Needed)

7. **Method chaining** - Requires rewriting chain formatting
8. **Line preservation** - Needs tracking original line breaks
9. **Edge cases with low match %** - Multiple interacting issues

## Essential Commands

```bash
# Before starting any work
just ready           # Clean build
just coverage        # See current status (may timeout, use command below instead)

# Check current conformance status (faster)
cargo run -p oxc_prettier_conformance 2>/dev/null | grep "compatibility:"

# While working
cargo run -p oxc_prettier_conformance -- --filter "test-name"  # Test specific file
just test           # Run all tests

# After each fix
cargo run -p oxc_prettier_conformance 2>/dev/null | grep "compatibility:"  # Verify improvement
git add -p  # Stage only your changes
git commit -m "fix(formatter): [brief description]"
```

## Key Files Reference

```
crates/oxc_formatter/src/
├── parentheses/expression.rs    # NeedsParentheses implementations ← Most fixes here
├── utils/mod.rs                 # is_expression_used_as_call_argument() utility
└── write/                       # Formatting implementations
    ├── mod.rs                   # Main write implementations
    └── array_expression.rs      # Array-specific formatting
```

## The Successful Pattern

Every fix so far follows this pattern:

```rust
impl<'a> NeedsParentheses<'a> for AstNode<'a, SomeExpression<'a>> {
    fn needs_parentheses(&self, f: &Formatter<'_, 'a>) -> bool {
        let parent = self.parent;
        
        // CHECK PARENT CONTEXT FIRST
        if /* some condition about parent */ {
            return true/false;  // Override default
        }
        
        // Keep existing logic
        existing_checks(...)
    }
}
```

## Available Utilities

- `is_expression_used_as_call_argument(span, parent)` - checks if expression is function arg
- `is_class_extends(parent, span)` - checks if in extends clause  
- `is_first_in_statement(...)` - checks if expression starts statement
- `expression_is_or_contains_call(expr)` - recursively checks for CallExpression/TaggedTemplate

**Pro tip:** Use `eprintln!("Parent: {:#?}", parent);` to debug parent context

## Success Metrics

✅ Each fix should:
- Increase the JS compatibility % 
- Not break any existing tests
- Match the Prettier output exactly

## Common Pitfalls

❌ **Don't** try to recreate AstKind::Argument
❌ **Don't** make broad changes - target specific issues
❌ **Don't** commit without testing the full suite
❌ **Don't** assume ParenthesizedExpression nodes exist (preserve_parens: false in tests)
✅ **Do** use existing utilities
✅ **Do** follow the successful pattern
✅ **Do** commit each fix separately
✅ **Do** implement for specific expression types (StaticMemberExpression, etc.)

## Git Workflow

1. Make your fix in the appropriate file (usually `parentheses/expression.rs`)
2. Test with: `cargo run -p oxc_prettier_conformance -- --filter "your-test"`
3. Verify overall improvement didn't break anything
4. Stage only your changes: `git add -p`
5. Commit with descriptive message: `git commit -m "fix(formatter): [what you fixed]"`

## Questions?

- For investigation notes: See OLD_CONFORMANCE_ANALYSIS.md (archived)
- For detailed analysis: See REMAINING_CONFORMANCE_TEST_ANALYSIS.md
- Current branch: `temp/fix-conformance-tests-take-two`
- Target branch for PR: `main`

---
*Last updated after fixing Angular test wrappers (JS: 500/699)*