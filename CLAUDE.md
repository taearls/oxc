# CLAUDE.md - AI Assistant Instructions for Oxc

## Project Overview

Oxc is a high-performance JavaScript/TypeScript toolchain written in Rust. It includes a parser, linter (oxlint), formatter, transformer, and minifier.

## Current Active Investigation

### 📁 Formatter Conformance Investigation

**External Repository**: https://github.com/taearls/oxc-debugging-tools\
**Branch**: `temp/resume-fixing-conformance-tests`\
**Task**: Fixing formatter regressions from AstKind::Argument removal

**Setup Investigation Tools**:

```bash
# Clone the debugging tools repository
git clone git@github.com:taearls/oxc-debugging-tools.git ~/code/oxc-debugging-tools

# Navigate to tools
cd ~/code/oxc-debugging-tools

# View investigation guide
cat START_HERE.md
```

**Key Resources in oxc-debugging-tools**:

- `START_HERE.md` - Entry point for newcomers
- `ACTION_PLAN.md` - Prioritized fixes to implement
- `CONFORMANCE_FIX_GUIDE.md` - Step-by-step fix guide
- `conformance_investigation_scripts.py` - Analysis tools
- `track_progress.sh` - Real-time progress monitoring

**Current Status**:

- Target: JS 525/699 (75.11%), TS 277/573 (48.34%)
- Current: JS 517/699 (73.96%), TS 274/573 (47.82%)
- Need: +8 JS tests, +3 TS tests

**Workflow**:

1. Make fixes in this Oxc repository
2. Test and validate changes
3. Commit source code changes to Oxc
4. Update progress in oxc-debugging-tools repository
5. Commit progress updates to debugging tools

## Critical Development Rules

### Setup Already Complete

- **DO NOT run `just init`** - All tools are already installed
- **DO NOT install Rust components** - clippy, rust-docs, rustfmt are already present
- **ALWAYS run `just ready`** as the final step after committing code

### Before Making Changes

1. **Format code**: Run `just fmt` after any code modifications
2. **Check code**: Run `just check` to verify compilation
3. **Run tests**: Execute `just test` for the affected crates
4. **Final verification**: Run `just ready` before marking any task complete

### Special Commands for Specific Changes

- **AST changes**: Run `just ast` to update generated files
- **Rust code changes**: Run `just conformance` for conformance tests
- **Minifier changes**: Run `just minsize` to update size snapshots
- **Parser changes**: Run `just allocs` to update allocation snapshots

## Code Modification Guidelines

### File Structure

- Main code in `crates/` directory
- Application binaries in `apps/`
- **NEVER edit files in `generated` subdirectories**

### Core Crates to Know

- `oxc_parser` - JS/TS parser
- `oxc_ast` - AST definitions
- `oxc_linter` - Linting engine/rules
- `oxc_semantic` - Semantic analysis
- `oxc_transformer` - Code transformation
- `oxc_codegen` - Code generation
- `oxc_minifier` - Minification
- `oxc_formatter` - Formatting
- `oxc_diagnostics` - Error reporting

### Performance Requirements

- **Avoid unnecessary allocations** - This is performance-critical code
- Use `oxc_allocator` for memory management
- Follow existing patterns for optimal performance

### Testing Requirements

- Tests are co-located with source code
- Integration tests in `tests/` directory
- Uses `insta` for snapshot testing
- **Always add tests for new features**

## Common Development Tasks

### Adding a Linting Rule

1. Create module in `crates/oxc_linter/src/rules/`
2. Implement using visitor pattern
3. Add tests in the same module
4. Register in appropriate category
5. Run `just fmt`, `just test`, `just conformance`, `just ready`

### Parser Modifications

1. Research grammar changes thoroughly
2. Update AST in `oxc_ast` if needed
3. Ensure all existing tests pass
4. Add comprehensive tests
5. Run `just allocs` to update snapshots
6. Run `just fmt`, `just test`, `just conformance`, `just ready`

### Working with Transformations

1. Study AST structure first
2. Use visitor pattern for traversal
3. Handle node ownership/allocator carefully
4. Test with various input patterns
5. Run `just fmt`, `just test`, `just conformance`, `just ready`

## Essential Commands Reference

```bash
# Primary development commands
just fmt         # Format code (ALWAYS after changes)
just check       # Check compilation
just test        # Run tests
just lint        # Run linting
just ready       # Final verification (ALWAYS before completing task)

# Specialized update commands
just ast         # Update generated AST files
just conformance # Run conformance tests
just minsize     # Update minifier snapshots
just allocs      # Update parser allocation snapshots

# Node.js development
pnpm build-dev   # Build Node.js bindings
pnpm test        # Run Node.js tests

# Conformance investigation (using external debugging tools)
cd ~/code/oxc-debugging-tools
python3 conformance_investigation_scripts.py  # Analyze test failures
./track_progress.sh  # Monitor real-time progress
```

## Running Examples

Most crates have examples:

```bash
cargo run -p <crate_name> --example <example_name> [filename]
```

Key examples:

- `cargo run -p oxc_parser --example parser` - Parsing demo
- `cargo run -p oxc_linter --example linter` - Linting demo
- `cargo run -p oxc_semantic --example semantic` - Semantic analysis
- `cargo run -p oxc_transformer --example transformer` - Transformation
- `cargo run -p oxc --example compiler --features="full"` - Full pipeline

## Important Notes

- This is a rapidly evolving project - APIs may change
- Maintain JavaScript/TypeScript standard compatibility
- Breaking changes need discussion and documentation
- Performance is critical for ALL changes
- Follow rustfmt config in `.rustfmt.toml`
- Use `oxc_diagnostics` for errors with source locations

## Verification Checklist

Before marking any task complete:

- [ ] Code formatted with `just fmt`
- [ ] Tests pass with `just test`
- [ ] Conformance tests pass with `just conformance`
- [ ] All checks pass with `just ready`
- [ ] Updated snapshots if needed (ast/minsize/allocs)
- [ ] Added tests for new functionality
