# Analysis and Improvements for Conformance Investigation Folder

## Critical Issues Found

### 1. Inconsistencies Between Documents

#### Issue: Conflicting Information About Arrow Currying Fix

- **CLAUDE.md** lists it as "FIXED in development branch"
- **CONFORMANCE_FIX_QUICK_REFERENCE.md** lists it as "ATTEMPTED"
- **Reality**: The fix was attempted but didn't improve test count

#### Issue: File Index Out of Date

- **CLAUDE.md** says it lists 4 files but folder now has 6 files
- Missing ACTION_PLAN.md and track_progress.sh from index

### 2. Missing Context for Newcomers

#### What's Missing:

- No explanation of what conformance tests are
- No explanation of what Prettier is
- No explanation of why we compare to Prettier
- No explanation of what a code formatter does
- No clear "START HERE" guide

### 3. Technical Concepts Not Explained

#### Undefined Terms:

- **AstKind::Argument**: What was it? Why was it removed?
- **Chain vs Single layout**: What do these mean?
- **NeedsParentheses trait**: What is this?
- **Format trait**: How does it work?

### 4. Missing Practical Information

#### Not Documented:

- How to see actual formatting differences (before/after)
- Where snapshot files are located
- How to run individual failing tests
- What the actual test files look like
- Visual examples of formatting issues

### 5. Incorrect Technical Details

#### In ACTION_PLAN.md:

```rust
// Incorrect - "expect" is not a wrapper function
matches!(ident.name.as_str(),
    "it" | "test" | "describe" | "beforeEach" | "afterEach" | 
    "beforeAll" | "afterAll" | "expect")  // ❌ expect doesn't wrap functions
```

## Recommended Improvements

### 1. Create START_HERE.md

A single entry point for newcomers that explains:

- What is Oxc?
- What is a code formatter?
- What are conformance tests?
- Why do we compare to Prettier?
- How to navigate this investigation

### 2. Add GLOSSARY.md

Define all technical terms:

- AST (Abstract Syntax Tree)
- AstKind::Argument
- Conformance
- Layout (Chain, Single, Group)
- Traits (NeedsParentheses, Format)

### 3. Add VISUAL_EXAMPLES.md

Show actual formatting differences:

```javascript
// Oxc (incorrect):
it(
  'test',
  async(
    () => {
      console.log('test');
    },
  ),
);

// Prettier (correct):
it(
  'test',
  async(() => {
    console.log('test');
  }),
);
```

### 4. Consolidate and Clarify

#### Merge Overlapping Content:

- CONFORMANCE_FIX_GUIDE.md and CONFORMANCE_FIX_QUICK_REFERENCE.md have duplicate info
- Consider merging into one comprehensive guide

#### Clear Navigation:

```
START_HERE.md → README.md → ACTION_PLAN.md → CONFORMANCE_FIX_GUIDE.md
                     ↓
                GLOSSARY.md (reference as needed)
```

### 5. Fix Technical Inaccuracies

#### Arrow Currying Status:

- Change all references to clarify it was attempted but unsuccessful
- Document why it didn't work

#### Test Wrapper Functions:

- Remove "expect" from wrapper detection suggestions
- Add correct Jest patterns like "it.each", "test.each"

### 6. Add Missing Documentation

#### For track_progress.sh:

- Add comments explaining what it does
- Document the color coding system
- Explain the log file format

#### For Python Scripts:

- Create separate SCRIPT_DOCUMENTATION.md
- Explain each function's purpose with examples

### 7. Create Test Running Guide

#### Document How To:

- View failing test source: `cat tasks/prettier_conformance/tests/format/js/[test].js`
- Run single test: `cargo run --bin oxc_prettier_conformance -- --filter "test-name"`
- Compare outputs: Side-by-side Oxc vs Prettier
- Understand snapshot files

## Priority Order for Improvements

1. **Fix inconsistencies** (High - prevents confusion)
2. **Create START_HERE.md** (High - critical for onboarding)
3. **Add visual examples** (High - understanding issues)
4. **Create glossary** (Medium - reference material)
5. **Consolidate guides** (Low - organizational)

## Validation Checklist

Before considering documentation complete:

- [ ] A developer with no Rust experience can understand the problem
- [ ] A developer can find and run a failing test in <5 minutes
- [ ] All technical terms are defined
- [ ] All code examples are correct and tested
- [ ] File index is accurate and up-to-date
- [ ] No conflicting information between documents
- [ ] Clear progression path through documentation
