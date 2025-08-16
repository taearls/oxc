# Visual Examples of Formatting Issues

This document shows actual formatting differences between Oxc and Prettier for the failing tests.

## 1. Angular Test Wrappers (FIXED ✅)

### Before Fix

```javascript
// Oxc output (incorrect):
it(
  'test',
  async(
    () => {
      console.log('test');
    },
  ),
);

// Prettier output (correct):
it(
  'test',
  async(() => {
    console.log('test');
  }),
);
```

### Issue

The arrow function argument was being broken onto multiple lines instead of staying inline.

### Fix Applied

Detected Angular wrappers (`async`, `fakeAsync`, `waitForAsync`) and kept arguments inline.

---

## 2. Decorated Classes (FIXED ✅)

### Before Fix

```javascript
// Oxc output (incorrect):
class Foo extends @decorator class Bar {} {
  // Missing parentheses around decorated class
}

// Prettier output (correct):
class Foo extends (@decorator class Bar {}) {
  // Parentheses added for clarity
}
```

### Issue

Decorated class expressions in extends clauses were missing parentheses.

---

## 3. Ternary Operators (NEEDS FIX ❌)

### Current Issues

#### Indentation Problem

```javascript
// Oxc output (incorrect):
const result = condition
  ? valueIfTrue
  : valueIfFalse;

// Prettier output (correct):
const result = condition
  ? valueIfTrue
  : valueIfFalse;
```

#### Nested Ternaries

```javascript
// Oxc output (incorrect):
const x = a
  ? b
    ? c
    : d
  : e;

// Prettier output (correct):
const x = a
  ? b
    ? c
    : d
  : e;
```

### Issue

Indentation levels are not being calculated correctly for ternary operators, especially nested ones.

---

## 4. Jest Test Patterns (NEEDS FIX ❌)

### jest-each Pattern

```javascript
// Oxc output (incorrect):
test.each([
  [1, 1, 2],
  [1, 2, 3],
])(
  'adds %i + %i to equal %i',
  (a, b, expected) => {
    expect(a + b).toBe(expected);
  },
);

// Prettier output (correct):
test.each([
  [1, 1, 2],
  [1, 2, 3],
])('adds %i + %i to equal %i', (a, b, expected) => {
  expect(a + b).toBe(expected);
});
```

### Issue

The test description and callback are being broken incorrectly.

---

## 5. Arrow Function Semicolons (NEEDS FIX ❌)

### Missing Semicolons

```javascript
// Oxc output (incorrect):
const fn = () => console.log('hello');
const other = 42;

// Prettier output (correct):
const fn = () => console.log('hello');
const other = 42;
```

### Issue

Semicolons not being inserted after arrow function expressions in certain contexts.

---

## 6. Comment Attachment (NEEDS FIX ❌)

### Comments in Wrong Position

```javascript
// Oxc output (incorrect):
function foo(
  arg1,
  arg2,
  /* comment */
) {
  // Comment attached to wrong node
}

// Prettier output (correct):
function foo(
  arg1,
  arg2, /* comment */
) {
  // Comment stays with its associated argument
}
```

### Issue

After removing AstKind::Argument, comments lost their proper attachment points.

---

## 7. Binary Expressions in Return (NEEDS FIX ❌)

### Parentheses Issue

```javascript
// Oxc output (incorrect):
return a + b * c;

// Prettier output (correct - when needed for clarity):
return (a + b) * c;
```

### Issue

Parentheses logic for binary expressions in return statements needs adjustment.

---

## 8. Arrow Function Currying (ATTEMPTED ❓)

### Deep Currying

```javascript
// Oxc output (incorrect):
const curry = a => b => c => d => a + b + c + d;

// Prettier output (correct):
const curry = a => b => c => d => a + b + c + d;
```

### Issue

Deeply curried arrow functions (3+ levels) should break after the last arrow.

### Note

Fix was attempted but didn't improve test results. Needs different approach.

---

## How to Test These Examples

1. **Create a test file**:

```bash
echo 'your test code here' > test.js
```

2. **See Oxc output**:

```bash
cargo run --bin oxc_formatter test.js
```

3. **See Prettier output**:

```bash
npx prettier test.js
```

4. **Compare side-by-side**:

```bash
diff <(cargo run --bin oxc_formatter test.js 2>/dev/null) \
     <(npx prettier test.js)
```

## Understanding the Pattern

Most issues stem from the same root cause:

- **Before**: `AstKind::Argument` helped identify when expressions were function arguments
- **After**: Without this marker, the formatter can't determine proper formatting context
- **Solution**: Add explicit detection for common patterns (test wrappers, special functions)

## Priority Based on Impact

1. **🔴 High Priority** (Many tests affected):
   - Ternary operators (9 tests)
   - Comments (22 tests)

2. **🟡 Medium Priority** (Moderate impact):
   - Jest patterns (4 tests)
   - Arrow semicolons (4 tests)

3. **🟢 Low Priority** (Few tests, high complexity):
   - Arrow currying
   - Binary expressions

Focus on high-priority items first for maximum impact on conformance scores.
