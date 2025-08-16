#!/bin/bash

# Script to track conformance test progress during fixing session
# Usage: ./track_progress.sh [test-filter]

echo "================================================"
echo "Oxc Formatter Conformance Progress Tracker"
echo "================================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get current git info
BRANCH=$(git branch --show-current)
COMMIT=$(git rev-parse --short HEAD)

echo "Branch: $BRANCH"
echo "Commit: $COMMIT"
echo ""

# Run conformance tests
if [ -z "$1" ]; then
    echo "Running full conformance suite..."
    OUTPUT=$(cargo run --bin oxc_prettier_conformance 2>&1)
else
    echo "Running conformance for filter: $1"
    OUTPUT=$(cargo run --bin oxc_prettier_conformance -- --filter "$1" 2>&1)
fi

# Extract results
JS_LINE=$(echo "$OUTPUT" | grep "js compatibility:")
TS_LINE=$(echo "$OUTPUT" | grep "ts compatibility:")

# Parse JS results
if [[ $JS_LINE =~ ([0-9]+)/([0-9]+) ]]; then
    JS_PASS=${BASH_REMATCH[1]}
    JS_TOTAL=${BASH_REMATCH[2]}
    JS_PCT=$(echo "scale=2; $JS_PASS * 100 / $JS_TOTAL" | bc)
    
    # Color code based on target (525)
    if [ $JS_PASS -ge 525 ]; then
        echo -e "${GREEN}✓ JavaScript: $JS_PASS/$JS_TOTAL ($JS_PCT%) - TARGET MET!${NC}"
    elif [ $JS_PASS -ge 517 ]; then
        echo -e "${YELLOW}⚠ JavaScript: $JS_PASS/$JS_TOTAL ($JS_PCT%) - Need $((525 - JS_PASS)) more${NC}"
    else
        echo -e "${RED}✗ JavaScript: $JS_PASS/$JS_TOTAL ($JS_PCT%) - Need $((525 - JS_PASS)) more${NC}"
    fi
fi

# Parse TS results
if [[ $TS_LINE =~ ([0-9]+)/([0-9]+) ]]; then
    TS_PASS=${BASH_REMATCH[1]}
    TS_TOTAL=${BASH_REMATCH[2]}
    TS_PCT=$(echo "scale=2; $TS_PASS * 100 / $TS_TOTAL" | bc)
    
    # Color code based on target (277)
    if [ $TS_PASS -ge 277 ]; then
        echo -e "${GREEN}✓ TypeScript: $TS_PASS/$TS_TOTAL ($TS_PCT%) - TARGET MET!${NC}"
    elif [ $TS_PASS -ge 274 ]; then
        echo -e "${YELLOW}⚠ TypeScript: $TS_PASS/$TS_TOTAL ($TS_PCT%) - Need $((277 - TS_PASS)) more${NC}"
    else
        echo -e "${RED}✗ TypeScript: $TS_PASS/$TS_TOTAL ($TS_PCT%) - Need $((277 - TS_PASS)) more${NC}"
    fi
fi

echo ""
echo "------------------------------------------------"

# Show progress from baseline
echo "Progress from baseline (main: JS 525, TS 277):"
if [ ! -z "$JS_PASS" ]; then
    JS_DIFF=$((JS_PASS - 525))
    if [ $JS_DIFF -ge 0 ]; then
        echo -e "  JS: ${GREEN}+$JS_DIFF${NC} tests"
    else
        echo -e "  JS: ${RED}$JS_DIFF${NC} tests"
    fi
fi

if [ ! -z "$TS_PASS" ]; then
    TS_DIFF=$((TS_PASS - 277))
    if [ $TS_DIFF -ge 0 ]; then
        echo -e "  TS: ${GREEN}+$TS_DIFF${NC} tests"
    else
        echo -e "  TS: ${RED}$TS_DIFF${NC} tests"
    fi
fi

echo ""

# Log to file for history
LOG_FILE="progress_log.txt"
echo "$(date '+%Y-%m-%d %H:%M:%S') | Commit: $COMMIT | JS: $JS_PASS/$JS_TOTAL | TS: $TS_PASS/$TS_TOTAL" >> $LOG_FILE
echo "Progress logged to $LOG_FILE"

# Success check
if [ "$JS_PASS" -ge 525 ] && [ "$TS_PASS" -ge 277 ]; then
    echo ""
    echo -e "${GREEN}🎉 TARGETS MET! Ready to merge.${NC}"
    echo ""
fi