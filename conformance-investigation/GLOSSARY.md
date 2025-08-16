# Glossary of Technical Terms

## Core Concepts

### AST (Abstract Syntax Tree)

A tree representation of code structure. Each node represents a construct in the code (like a function, variable, or expression).

**Example**: The code `a + b` becomes:

```
BinaryExpression {
  operator: "+",
  left: Identifier("a"),
  right: Identifier("b")
}
```

### AstKind::Argument

A **removed** AST node type that previously wrapped expressions when used as function arguments. Its removal is the root cause of our formatting issues.

**Before removal**:

```rust
CallExpression {
  callee: "foo",
  arguments: [
    Argument(Expression)  // Wrapped in Argument node
  ]
}
```

**After removal**:

```rust
CallExpression {
  callee: "foo", 
  arguments: [
    Expression  // Direct expression, no wrapper
  ]
}
```

### Conformance

The degree to which Oxc's output matches Prettier's output. Measured as a percentage of tests that produce identical formatting.

### Formatter

A tool that automatically adjusts code style (spacing, line breaks, parentheses) according to predefined rules.

## Formatter Concepts

### Layout Types

Different strategies for organizing code output:

- **Chain Layout**: Used for chained operations like `a.b().c().d()`
- **Single Layout**: Used for simple, single-line expressions
- **Group Layout**: Used for grouped items that can break together

### NeedsParentheses Trait

A Rust trait (interface) that determines when an expression needs parentheses based on its context.

**Example**: `(a + b) * c` needs parentheses around addition due to operator precedence.

### Format Trait

A Rust trait that defines how each AST node should be converted to formatted text output.

## Rust Concepts

### Trait

Similar to an interface in other languages. Defines a set of methods that types can implement.

### impl (Implementation)

Provides the actual code for trait methods. Example: `impl Format for ArrowFunctionExpression`

### matches! Macro

Rust pattern matching macro for checking if a value matches specific patterns.

```rust
matches!(expr, Expression::Identifier(_))  // Returns true/false
```

## Testing Concepts

### Snapshot Testing

Testing approach where expected output is stored in "snapshot" files and compared against actual output.

### Test Wrapper Functions

Special functions used in testing frameworks that need specific formatting:

- **Angular**: `async()`, `fakeAsync()`, `inject()`, `waitForAsync()`
- **Jest**: `it()`, `test()`, `describe()`, `beforeEach()`, `afterEach()`

## File Locations

### Crate

A Rust package. Oxc has multiple crates like `oxc_formatter`, `oxc_parser`, etc.

### Generated Files

Files automatically created by build tools. Should never be edited manually. Found in `generated/` subdirectories.

## Commands and Tools

### just

A command runner (like Make). `just fmt` runs the format command defined in justfile.

### cargo

Rust's build tool and package manager. Similar to npm for JavaScript.

### ripgrep (rg)

Fast text search tool. Better than grep for searching code.

## Formatting Specific Terms

### Parentheses Logic

Rules determining when expressions need parentheses to maintain correct precedence or clarity.

### Comment Attachment

Process of associating comments with the appropriate AST nodes for proper placement in formatted output.

### Indentation Logic

Rules for how many spaces/tabs to indent nested structures.

### Line Breaking

Decisions about where to split long lines of code for readability.

### Currying

Writing functions that return other functions. Affects arrow function formatting:

```javascript
const curry = a => b => c => a + b + c;
```

## Progress Tracking Terms

### Match Percentage

Percentage of a test file's output that matches Prettier's output. 100% means perfect match.

### Quick Wins

Tests with >85% match that likely need only minor fixes.

### Regression

When a previously passing test starts failing, or when match percentage decreases.

## Investigation Specific

### Development Branch

`temp/resume-fixing-conformance-tests` - The branch containing the AstKind::Argument removal.

### Main Branch

The stable branch we're comparing against for baseline conformance levels.

### Commit Hash

Short identifier for a Git commit (e.g., `883108d18`). Used to track which changes fixed which tests.

## Debugging Terms

### Minimal Test Case

Smallest possible code example that reproduces an issue. Easier to debug than full test files.

### Diff

Comparison showing differences between two files or outputs. `+` shows additions, `-` shows removals.
