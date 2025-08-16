# Conformance Test Documentation

## Active Documents (Use These!)

### 📘 `CONFORMANCE_FIX_GUIDE.md` 
**Main guide for fixing conformance tests**
- Current status and gaps
- Step-by-step instructions for the next fix
- Complete priority list of all remaining fixes
- Detailed examples and patterns

### 📋 `CONFORMANCE_FIX_QUICK_REFERENCE.md`
**Quick reference for experienced developers**
- One-page summary
- Copy-paste commands
- The exact fix pattern
- Quick status check

## Other Documents

### 📚 `OLD_CONFORMANCE_ANALYSIS.md`
**Archived investigation notes** (1300+ lines)
- Historical analysis
- Previous attempts and learnings  
- Detailed investigation of complex issues
- Only consult if you need deep context

### 📝 `conformance_test_plan.md`
**Original test plan**
- Initial approach (completed)
- Historical reference only

## Which Document Should I Use?

- **Starting a fix?** → Use `CONFORMANCE_FIX_GUIDE.md`
- **Need quick commands?** → Use `CONFORMANCE_FIX_QUICK_REFERENCE.md`  
- **Stuck on something complex?** → Check `OLD_CONFORMANCE_ANALYSIS.md`
- **Just want status?** → Run `just coverage | grep "compat"`

## Current Status (Live)

Run this command to see current test status:
```bash
just coverage | grep "compatibility:"
```

Target: JS 505/699 (72.25%) - matching main branch