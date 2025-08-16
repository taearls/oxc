# Conformance Investigation Folder

This folder contains all documentation, tools, and knowledge accumulated during the investigation and fixing of Oxc formatter conformance test regressions.

## Purpose

The `temp/resume-fixing-conformance-tests` branch contains a major refactoring that removed `AstKind::Argument` from the AST, causing numerous formatter regressions. This folder helps track:

1. What has been fixed
2. What still needs fixing
3. How to approach remaining issues
4. Tools to analyze and prioritize work

## Quick Start

```bash
# Analyze current test failures
python3 conformance_investigation_scripts.py

# Check detailed fix guide
cat CONFORMANCE_FIX_GUIDE.md

# Review accumulated knowledge
cat CLAUDE.md
```

## Files

### 📚 Documentation (Read in Order)

- **START_HERE.md** - Entry point for newcomers with no context
- **README.md** - This file - overview and current status
- **ACTION_PLAN.md** - Prioritized execution plan with specific fixes
- **VISUAL_EXAMPLES.md** - See actual formatting differences (before/after)
- **CONFORMANCE_FIX_GUIDE.md** - Detailed implementation guide
- **CONFORMANCE_FIX_QUICK_REFERENCE.md** - Quick lookup reference
- **GLOSSARY.md** - Technical term definitions
- **CLAUDE.md** - Deep technical knowledge base

### 🔧 Tools

- **conformance_investigation_scripts.py** - Python analysis toolkit
- **track_progress.sh** - Bash script for real-time progress tracking

### 📋 Meta Documentation

- **ANALYSIS_AND_IMPROVEMENTS.md** - Analysis of documentation quality

## Current Progress

| Status        | JavaScript       | TypeScript       |
| ------------- | ---------------- | ---------------- |
| **Target**    | 525/699 (75.11%) | 277/573 (48.34%) |
| **Current**   | 517/699 (73.96%) | 274/573 (47.82%) |
| **Remaining** | +8 tests         | +3 tests         |

## For New Contributors

**⚠️ New to this investigation? Start with START_HERE.md for complete context!**

1. Read **START_HERE.md** first if you have no context
2. Use the Python scripts to analyze current failures
3. Follow **ACTION_PLAN.md** for prioritized fixes
4. Reference **CONFORMANCE_FIX_GUIDE.md** for implementation details
5. Check **GLOSSARY.md** when you encounter unfamiliar terms

## Key Insight

All regressions stem from removing `AstKind::Argument` from the AST. Most fixes involve:

- Detecting when expressions are used as arguments without the AST node
- Special-casing test framework patterns
- Fixing comment attachment points
