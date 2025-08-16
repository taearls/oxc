# 🚀 START HERE - Conformance Investigation Guide

Welcome! This guide will help you understand and fix formatter conformance issues in Oxc, even if you have no prior context.

## What You Need to Know

### What is Oxc?

Oxc is a high-performance JavaScript/TypeScript toolchain written in Rust. Think of it as a faster alternative to tools like ESLint, Prettier, and Babel.

### What is a Code Formatter?

A formatter automatically adjusts code style (spacing, line breaks, parentheses) to follow consistent rules. For example:

```javascript
// Before formatting:
const x = a ? b : c;

// After formatting:
const x = a ? b : c;
```

### What are Conformance Tests?

Conformance tests verify that Oxc's formatter produces the same output as Prettier (the industry standard formatter). We run thousands of test files through both formatters and compare outputs.

### Why This Investigation?

The branch `temp/resume-fixing-conformance-tests` made a major change: it removed `AstKind::Argument` from the Abstract Syntax Tree (AST). This broke the formatter's ability to recognize when expressions are used as function arguments, causing formatting regressions.

## The Problem in Simple Terms

Imagine the formatter needs to know: "Is this piece of code an argument to a function?"

**Before**: The AST had a special `Argument` node type that made this obvious.
**After**: Arguments look identical to regular expressions, making detection difficult.

This affects formatting decisions like:

- Should parentheses be added?
- Should line breaks be inserted?
- How should comments be attached?

## Your Mission

**Goal**: Fix formatter tests to match main branch performance

- **Current**: JS 517/699 (73.96%), TS 274/573 (47.82%)
- **Target**: JS 525/699 (75.11%), TS 277/573 (48.34%)
- **Needed**: +8 JavaScript tests, +3 TypeScript tests

## Quick Start Commands

```bash
# 1. Check current status
cargo run --bin oxc_prettier_conformance 2>&1 | grep "compatibility:"

# 2. See what's failing
python3 conformance_investigation_scripts.py

# 3. Track progress in real-time
./track_progress.sh

# 4. Test a specific fix
cargo run --bin oxc_prettier_conformance -- --filter "test-name"
```

## Navigation Guide

📖 **Read in this order:**

1. **This file** - Basic context (you are here)
2. **README.md** - Overview of investigation folder
3. **ACTION_PLAN.md** - Specific fixes to implement
4. **CONFORMANCE_FIX_GUIDE.md** - Detailed implementation guide

🔧 **Tools available:**

- **conformance_investigation_scripts.py** - Analyze failures
- **track_progress.sh** - Monitor improvements

📚 **Reference when needed:**

- **CLAUDE.md** - Deep technical knowledge base
- **CONFORMANCE_FIX_QUICK_REFERENCE.md** - Quick lookup

## Understanding Test Output

When you run conformance tests, you'll see:

```
js compatibility: 517/699 (73.96%)
ts compatibility: 274/573 (47.82%)
```

This means:

- JavaScript: 517 tests passing out of 699 total (73.96% match with Prettier)
- TypeScript: 274 tests passing out of 573 total (47.82% match with Prettier)

## Finding and Running Failing Tests

```bash
# 1. Find a failing test in the snapshot
cat tasks/prettier_conformance/snapshots/prettier.js.snap.md | grep "Failed" -A 20

# 2. View the test source
cat tasks/prettier_conformance/tests/format/js/arrows/curried.js

# 3. See Oxc's output
cargo run --bin oxc_formatter tasks/prettier_conformance/tests/format/js/arrows/curried.js

# 4. See Prettier's output
npx prettier tasks/prettier_conformance/tests/format/js/arrows/curried.js

# 5. Compare side by side
diff <(cargo run --bin oxc_formatter test.js) <(npx prettier test.js)
```

## Making Your First Fix

1. **Pick a "quick win"** from ACTION_PLAN.md (tests with >85% match)
2. **Locate the relevant code** using the file map in guides
3. **Make minimal changes** - small fixes are easier to verify
4. **Test your fix** on the specific failing test
5. **Verify no regressions** with full test suite
6. **Commit with descriptive message**

## Common Pitfalls to Avoid

❌ **Don't commit snapshot files** (*.snap.md)
❌ **Don't make large changes** without testing
❌ **Don't assume fixes** - always verify with tests
❌ **Don't skip `just ready`** before marking complete

## Getting Help

- **AST Structure**: `cargo run -p oxc_parser --example parser test.js`
- **Prettier Output**: https://prettier.io/playground/
- **Pattern Search**: `rg "pattern" crates/oxc_formatter/src/`

## Success Looks Like

✅ All target numbers reached (JS 525, TS 277)
✅ No regressions in previously passing tests
✅ Clean commit history with descriptive messages
✅ Documentation updated with findings

---

**Ready to start?** Continue to README.md for the investigation overview, then check ACTION_PLAN.md for specific tasks to tackle!
