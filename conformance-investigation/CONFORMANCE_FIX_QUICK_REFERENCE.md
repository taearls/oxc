# Conformance Fix Quick Reference

## ⚠️ ROOT CAUSE: AstKind::Argument Removal

Branch `temp/resume-fixing-conformance-tests` removed `AstKind::Argument` from AST.
This broke argument detection throughout the formatter.

## Progress Tracking (with commit hashes)

| Commit      | JS Tests    | TS Tests    | What Fixed                    |
| ----------- | ----------- | ----------- | ----------------------------- |
| `ca46ab9f7` | 513/699     | 274/573     | Starting point (before fixes) |
| `1a9e0a154` | 514/699     | 274/573     | Decorator fix (+1 JS)         |
| `883108d18` | 517/699     | 274/573     | Angular wrappers (+3 JS)      |
| **Target**  | **525/699** | **277/573** | Need +8 JS, +3 TS more        |

## ✅ Completed Fixes

### 1. Decorator Fix (DONE)

```rust
// crates/oxc_formatter/src/parentheses/expression.rs
// In ClassExpression impl:
if !self.decorators.is_empty() && is_class_extends(parent, self.span) {
    return true;
}
```

### 2. Arrow Currying Fix (ATTEMPTED)

- Modified chain layout but didn't improve test count
- May need different approach

### 3. Angular Wrappers (DONE)

```rust
// crates/oxc_formatter/src/write/call_arguments.rs
// Keep async(), fakeAsync(), etc. arguments inline:
if is_angular_wrapper && self.len() == 1 && self.first().is_some_and(|arg| 
    matches!(arg.as_ref(), Argument::ArrowFunctionExpression(_))
) {
    // Don't break the arguments
}
```

## ❌ Remaining Issues (Need +8 JS, +3 TS)

### Potential areas to investigate:

- `angularjs_inject.js` still failing (91.53% match)
- Function composition tests
- Jest test declarations
- Ternary operators (many failures)
- TypeScript-specific issues

## Quick Commands

```bash
# Check current status
cargo run --bin oxc_prettier_conformance 2>&1 | grep "compatibility:"

# Test specific file
cargo run --bin oxc_prettier_conformance -- --filter "test-name"

# After making a fix, ALWAYS:
git add crates/oxc_formatter/src/[file].rs
git commit -m "fix(formatter): [description]"
cargo run --bin oxc_prettier_conformance 2>&1 | grep "compatibility:" > progress.txt
echo "Commit: $(git rev-parse --short HEAD)" >> progress.txt
cat progress.txt

# Full check
just ready
```

## File Map

| Component          | File                                 | Status               |
| ------------------ | ------------------------------------ | -------------------- |
| Argument detection | `utils/mod.rs`                       | ✅ Exists            |
| Decorator parens   | `parentheses/expression.rs`          | ✅ Fixed             |
| Call arguments     | `write/call_arguments.rs`            | ✅ Angular fix added |
| Arrow formatting   | `write/arrow_function_expression.rs` | ⚠️ Attempted          |

## How to Track Progress

After EVERY commit:

1. Run conformance test
2. Record the results with commit hash
3. Update this document if improvement
4. If no improvement, consider reverting

Example:

```bash
# After commit
cargo run --bin oxc_prettier_conformance 2>&1 | grep "compatibility:"
# js compatibility: 517/699 (73.96%)
# ts compatibility: 274/573 (47.82%)
git rev-parse --short HEAD
# 883108d18
```

## Success Criteria

✅ JS: 525/699 (75.11%)
✅ TS: 277/573 (48.34%)
✅ No tests that passed in main are failing
