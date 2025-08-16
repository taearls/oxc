# Action Plan - Conformance Test Fixes

## Goal

Restore conformance to main branch levels:

- JavaScript: 525/699 (75.11%) - Need +8 tests
- TypeScript: 277/573 (48.34%) - Need +3 tests

## Prioritized Tasks

### 🔴 Priority 1: Quick Wins (High confidence, fast fixes)

#### 1.1 Fix angularjs_inject.js (91.53% → 100%)

**File**: `crates/oxc_formatter/src/write/call_arguments.rs`
**Action**: Add "inject" to Angular wrapper detection

```rust
// Current detection misses "inject"
matches!(ident.name.as_str(), 
    "async" | "inject" | "fakeAsync" | "waitForAsync")
```

**Impact**: +1 JS test

#### 1.2 Fix binary-expressions/return.js (90% → 100%)

**File**: `crates/oxc_formatter/src/parentheses/expression.rs`
**Action**: Check parentheses logic for binary expressions in return statements
**Impact**: +1 JS test

### 🟠 Priority 2: High Impact Areas

#### 2.1 Ternary Operators (9 tests at ~31% match)

**Files**:

- `crates/oxc_formatter/src/format/conditional_expression.rs`
- `crates/oxc_formatter/src/parentheses/expression.rs`

**Issues to investigate**:

- `js/ternaries/indent.js` (6.2% match)
- `js/ternaries/await-expression.js` (14.3% match)
- `js/ternaries/nested-in-condition.js` (23.9% match)

**Likely cause**: Indentation and parentheses handling broken after AstKind::Argument removal

**Test case**:

```javascript
// Should indent properly
const x = condition
  ? firstValue
  : secondValue;

// Nested ternaries
const y = a
  ? b
    ? c
    : d
  : e;
```

**Impact**: Could fix 3-5 JS tests

#### 2.2 Jest Test Patterns

**File**: `crates/oxc_formatter/src/write/call_arguments.rs`
**Tests**:

- `jest-each.js` (67.7% match)
- `jest-each-template-string.js` (27.8% match)

**Action**: Extend test wrapper detection to handle Jest patterns:

```rust
// Add Jest test functions that wrap callbacks
matches!(ident.name.as_str(),
    "it" | "test" | "describe" | "beforeEach" | "afterEach" | 
    "beforeAll" | "afterAll")
// Note: "expect" is NOT a wrapper - it's an assertion function
```

**Impact**: +2 JS tests

### 🟡 Priority 3: Systematic Issues

#### 3.1 Arrow Function Semicolons

**File**: `crates/oxc_formatter/src/write/arrow_function_expression.rs`
**Tests**: Multiple at 0% match
**Action**: Check semicolon insertion logic for arrow functions

#### 3.2 Comment Attachment

**File**: `crates/oxc_formatter/src/comments.rs`
**Issue**: Comment attachment points changed after AstKind::Argument removal
**Tests**: 22 comment tests failing

This is complex and may require significant investigation.

### 🟢 Priority 4: TypeScript Fixes

#### 4.1 TypeScript Satisfies Operator

**Files**: Check TypeScript-specific formatting
**Tests**: 13 tests in typescript/satisfies-operators
**Impact**: Could fix 1-2 TS tests

## Execution Strategy

### Phase 1: Quick Wins (Target: +2-3 tests)

1. Fix angularjs_inject.js
2. Fix binary-expressions/return.js
3. Test and commit each fix separately

### Phase 2: Ternary Operators (Target: +3-4 tests)

1. Create minimal test cases
2. Debug indentation logic
3. Fix parentheses handling
4. Test comprehensively

### Phase 3: Test Patterns (Target: +2 tests)

1. Extend Jest wrapper detection
2. Test with various Jest patterns
3. Ensure no regressions

### Phase 4: Final Push (Target: Remaining tests)

1. Arrow semicolons
2. Any remaining quick fixes
3. TypeScript if needed

## Verification Commands

```bash
# After each fix
cargo run --bin oxc_prettier_conformance -- --filter "test-name"

# Check overall progress
cargo run --bin oxc_prettier_conformance 2>&1 | grep "compatibility:"

# Before committing
just fmt
just test
just ready

# Track progress
cd conformance-investigation
python3 conformance_investigation_scripts.py
```

## Success Criteria

✅ JavaScript: 525/699 (75.11%)
✅ TypeScript: 277/573 (48.34%)
✅ No regressions in previously passing tests
✅ All fixes documented with commit hashes

## Risk Mitigation

- Test each fix in isolation
- Commit frequently with descriptive messages
- Run full test suite before moving to next fix
- Keep fixes minimal and targeted
- Document any discovered patterns

## Notes

- All issues stem from AstKind::Argument removal
- Focus on systematic fixes over individual test fixes
- Quick wins build momentum and confidence
- Comment issues are complex - save for last if time permits
