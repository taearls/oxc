# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Essential Commands

### Development Commands

- `just ready` - Run the complete CI pipeline (format, check, test, lint, doc, ast)
- `just check` - Run cargo check
- `just test` - Run all tests with all features
- `just lint` - Run clippy with warnings as errors
- `just fmt` - Format all code (cargo fmt + dprint)
- `just fix` - Auto-fix clippy issues and format code

### Building and Testing Tools

- `just oxlint` - Build oxlint in release mode
- `just example <tool> [args]` - Run examples for parser, formatter, linter
- `just coverage` - Run conformance tests (Test262, Babel, TypeScript)
- `just benchmark` - Run performance benchmarks

### Linter Rule Development

- `just new-rule <name>` - Generate new ESLint rule
- `just new-ts-rule <name>` - Generate new TypeScript rule
- `just new-react-rule <name>` - Generate new React rule
- `just new-oxc-rule <name>` - Generate new OXC-specific rule

### AST and Code Generation

- `just ast` - Regenerate AST boilerplate (run after AST changes)

## Project Architecture

### Core Structure

The project is organized as a Rust workspace with these key components:

**Core Crates:**

- `crates/oxc_parser/` - JavaScript/TypeScript parser (fastest Rust-based parser)
- `crates/oxc_linter/` - Linting engine with 430+ rules
- `crates/oxc_ast/` - AST definitions with memory arena allocation
- `crates/oxc_semantic/` - Semantic analysis and symbol resolution
- `crates/oxc_transformer/` - Code transformation (TypeScript, React, ES6+)
- `crates/oxc_minifier/` - JavaScript minification
- `crates/oxc_codegen/` - Code generation from AST
- `crates/oxc_formatter/` - Code formatting (Prettier-compatible)

**Applications:**

- `apps/oxlint/` - CLI linter application
- `napi/` - Node.js bindings for various tools

**Key Directories:**

- `crates/` - Core Rust crates containing the main functionality
- `apps/` - Application binaries (currently contains `oxlint`)
- `napi/` - Node.js bindings for JavaScript/TypeScript integration
- `npm/` - npm packages and Node.js-related code
- `tasks/` - Development tools, testing, and automation scripts
- `editors/` - Editor integrations (VS Code extension, etc.)

**Note**: Sub-directories named `generated` are code generated - do not edit files in these directories.

### Linter Rules Organization

Rules are categorized by source:

- `eslint/` - Core ESLint rules
- `typescript/` - TypeScript-ESLint rules
- `react/` - React plugin rules
- `jest/` - Jest plugin rules
- `oxc/` - OXC-specific performance rules

### Key Design Principles

- **Performance-first**: AST allocated in memory arena (bumpalo) for speed
- **Memory efficiency**: Short strings inlined with CompactString
- **Correctness**: Extensive test infrastructure with Test262, Babel, TypeScript conformance
- **Type safety**: Uses distinct AST node types vs generic estree nodes

## Development Workflow

### Before Starting Work

1. Run `just ready` to ensure clean state
2. For linter work, understand that rules follow this pattern:
   ```rust
   fn diagnostic_function(span: Span) -> OxcDiagnostic { /* */ }

   #[derive(Debug, Default, Clone)]
   pub struct RuleName;

   declare_oxc_lint!(/* metadata */);

   impl Rule for RuleName {
       fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) { /* */ }
   }
   ```

### Testing

- Each rule must have comprehensive tests using the `Tester` framework
- Include both passing and failing test cases
- Sometimes includes fix tests, which would apply a linting fix to a given string of source code
- Test edge cases and complex scenarios
- Run `cargo test --all-features` to verify all tests pass
- If a single test fails during the test run for a given source code file, then all test execution immediately stops. Therefore, if one failing test suddenly passes, it does not necessarily mean all tests are passing until we have passed every test in the file.

### Code Quality Standards

From `.cursor/rules/`:

- Extract complex logic into focused helper functions
- Use guard clauses instead of deep nesting
- Prefer enums over runtime string checking
- Use `Option`/`Result` with `?` operator for error handling
- Optimize for performance - lint rules are called frequently

### Performance Considerations

- Avoid unnecessary allocations in hot paths
- Use `&str` instead of `String` when possible
- Consider algorithmic complexity
- The parser/linter prioritizes speed: 50-100x faster than ESLint

## Handling Regressions and Breaking Changes

When working with regressions caused by breaking changes (e.g., AST modifications, API changes), follow this proven approach:

### ✅ Preferred: Targeted Fixes
1. **Identify specific symptoms** - Use concrete failing examples to understand the problem
2. **Trace behavioral differences** - Find exactly what changed in behavior, not just what was removed
3. **Implement minimal compensating logic** - Add just enough code to restore expected behavior
4. **Validate scope** - Ensure changes only affect the broken functionality
5. **Measure results** - Confirm the fix resolves the regression without side effects

### ❌ Avoid: Sweeping Infrastructure Changes
- Don't try to restore removed low-level components (e.g., reinstating deleted AST nodes)
- Avoid modifying core generation logic that affects multiple systems
- Don't make changes that require deep understanding of complex foundational code

### Why This Approach Works
- **Lower risk**: Changes are isolated and predictable
- **Easier debugging**: Clear cause-and-effect relationship
- **Better maintainability**: Future developers can understand the fix's purpose
- **Faster implementation**: No need to modify complex foundational systems

**Example**: When removing `AstKind::Argument` broke formatter parentheses, the solution was adding targeted detection logic for function arguments rather than trying to restore the deleted AST node type.

## Code Navigation Tips

### Understanding the AST

- Start with `oxc_ast` to understand the AST structure
- Look at `oxc_ast_visit` for traversal patterns
- Check `oxc_semantic` for symbol resolution and scoping

### Testing Patterns

- Unit tests are co-located with source code
- Integration tests in `tests/` directories
- Snapshot testing using `insta` crate for AST and output verification
- Conformance testing against real-world codebases

### Error Handling

- Use `oxc_diagnostics` for user-facing errors
- Follow the diagnostic pattern for error reporting
- Include source location information in diagnostics

## Important Notes

- Run `just ast` after making changes to AST definitions
- The project uses a memory arena (`bumpalo`) for fast AST allocation/deallocation
- Semantic analysis and symbol resolution are separate from parsing for performance
- All tools aim to be production-ready with extensive conformance testing
- This is a high-performance project - consider memory allocation patterns and algorithmic complexity
- Compatibility with JavaScript/TypeScript standards is critical
- The project maintains high code quality standards with rustfmt and clippy configuration
