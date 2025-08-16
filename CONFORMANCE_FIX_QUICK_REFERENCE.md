# Quick Fix Reference

## Status
**Current:** JS 493/699 (70.53%) | **Target:** JS 505/699 (72.25%) | **Gap:** 12 tests

## Next Task: New Expression Calls
```bash
# 1. Test it's broken
cargo run -p oxc_prettier_conformance -- --filter "js/new-expression/call.js"

# 2. Fix in: crates/oxc_formatter/src/parentheses/expression.rs
# Add to NewExpression's needs_parentheses():
if let AstNodes::CallExpression(call) = parent {
    if call.callee.span() == self.span() {
        return true;  // new (Factory())()
    }
}

# 3. Test it works
cargo run -p oxc_prettier_conformance -- --filter "js/new-expression/call.js"
just coverage | grep "JS"  # Should show 494/699

# 4. Commit
git add crates/oxc_formatter/src/parentheses/expression.rs
git commit -m "fix(formatter): add parentheses for new expressions used as callees"
```

## Fix Order (Easiest First)
1. ✅ ObjectExpression - DONE (26 tests)
2. ✅ ClassExpression - DONE (3 tests)  
3. **→ NewExpression** - DO THIS NEXT (1 test)
4. Require statements (1 test)
5. Test frameworks (3 tests)
6. Arrow functions (2 tests)
7. Function composition (1 test)

## The Pattern That Works
```rust
impl<'a> NeedsParentheses<'a> for AstNode<'a, YourExpression<'a>> {
    fn needs_parentheses(&self, f: &Formatter<'_, 'a>) -> bool {
        // 1. Check parent context FIRST
        if is_expression_used_as_call_argument(self.span, parent) {
            return false;  // or true, depending on the case
        }
        // 2. Keep existing logic
        existing_checks(...)
    }
}
```

## Commands
```bash
just ready                          # Before starting
just coverage | grep "compat"      # Check progress
cargo run -p oxc_prettier_conformance -- --filter "test-name"  # Test specific
```

## Key Files
- `parentheses/expression.rs` - Most fixes here
- `utils/mod.rs` - Helper functions
- `write/*.rs` - Formatting logic

## See `CONFORMANCE_FIX_GUIDE.md` for detailed instructions