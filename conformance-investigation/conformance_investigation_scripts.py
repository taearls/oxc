#!/usr/bin/env python3
"""
Conformance Investigation Scripts for Oxc Formatter

This file contains various Python scripts used during the investigation and fixing
of Oxc formatter conformance test regressions. These scripts were developed to
analyze test failures, identify patterns, and track progress while fixing issues
caused by the removal of AstKind::Argument from the AST.

Author: Claude (Anthropic)
Date: 2025-08-16
Project: Oxc Formatter Conformance Fixes
Branch: temp/resume-fixing-conformance-tests
"""

import re
import subprocess
from collections import defaultdict
from typing import Dict, List, Tuple, Optional
import json


# ==============================================================================
# SECTION 1: TEST RESULT PARSING
# ==============================================================================

def parse_test_line(line: str) -> Tuple[Optional[str], Optional[float]]:
    """
    Parse a test result line from conformance test output to extract test name and match percentage.
    
    Args:
        line: A line from the conformance test output, e.g., "| js/arrows/call.js | 💥💥 | 92.04% |"
    
    Returns:
        Tuple of (test_path, match_percentage) or (None, None) if parsing fails
    
    Use Case:
        Used to extract structured data from the prettier conformance snapshot files
        to analyze which tests are failing and by how much.
    """
    match = re.match(r'\| ([\w/\-\.]+) \| .* \| ([\d\.]+)% \|', line)
    if match:
        return match.group(1), float(match.group(2))
    return None, None


def read_test_results(filename: str) -> Dict[str, float]:
    """
    Read all test results from a conformance snapshot file.
    
    Args:
        filename: Path to the snapshot file (e.g., prettier.js.snap.md)
    
    Returns:
        Dictionary mapping test paths to their match percentages
    
    Use Case:
        Used to load complete test results for comparison between branches
        or tracking progress over time.
    """
    results = {}
    with open(filename, 'r') as f:
        for line in f:
            test, percentage = parse_test_line(line)
            if test:
                results[test] = percentage
    return results


# ==============================================================================
# SECTION 2: REGRESSION ANALYSIS
# ==============================================================================

def find_regressions(main_results: Dict[str, float], 
                    dev_results: Dict[str, float]) -> List[Tuple[str, float, float]]:
    """
    Find tests that regressed between main branch and development branch.
    
    Args:
        main_results: Test results from main branch
        dev_results: Test results from development branch
    
    Returns:
        List of tuples (test_path, main_percentage, dev_percentage) for regressed tests
    
    Use Case:
        Used to identify which specific tests got worse after the AstKind::Argument
        removal, helping prioritize which issues to fix first.
    """
    regressions = []
    
    # Check tests that are failing in dev
    for test in dev_results:
        if test not in main_results:
            # Test passes in main (not in failed list) but fails in dev
            regressions.append((test, 100.0, dev_results[test]))
        elif main_results[test] > dev_results[test]:
            # Test got worse
            regressions.append((test, main_results[test], dev_results[test]))
    
    return regressions


def group_by_category(tests: List[Tuple[str, float, float]]) -> Dict[str, List[Tuple[str, float, float]]]:
    """
    Group test regressions by their category (first directory in path).
    
    Args:
        tests: List of test regressions
    
    Returns:
        Dictionary mapping categories to lists of tests in that category
    
    Use Case:
        Used to understand which areas of the formatter are most affected
        and to identify systematic issues affecting multiple related tests.
    """
    categories = defaultdict(list)
    for test, main_pct, dev_pct in tests:
        # Extract category from path like "js/arrows/call.js" -> "arrows"
        parts = test.split('/')
        if len(parts) >= 2:
            category = parts[1]
            categories[category].append((test, main_pct, dev_pct))
    return dict(categories)


# ==============================================================================
# SECTION 3: PROGRESS TRACKING
# ==============================================================================

def track_progress_after_commit():
    """
    Track conformance test progress after making a commit.
    
    Use Case:
        Run this after each fix to record the current state and commit hash.
        Helps maintain a history of improvements for documentation.
    """
    # Get current commit hash
    commit = subprocess.run(['git', 'rev-parse', '--short', 'HEAD'], 
                          capture_output=True, text=True).stdout.strip()
    
    # Run conformance tests
    result = subprocess.run(['cargo', 'run', '--bin', 'oxc_prettier_conformance'], 
                          capture_output=True, text=True, stderr=subprocess.STDOUT)
    
    # Extract compatibility lines
    js_compat = None
    ts_compat = None
    for line in result.stdout.split('\n'):
        if 'js compatibility:' in line:
            js_compat = line.strip()
        elif 'ts compatibility:' in line:
            ts_compat = line.strip()
    
    if js_compat and ts_compat:
        print(f"Commit: {commit}")
        print(f"  {js_compat}")
        print(f"  {ts_compat}")
        
        # Return structured data for documentation
        return {
            'commit': commit,
            'javascript': js_compat.split(':')[1].strip(),
            'typescript': ts_compat.split(':')[1].strip()
        }
    
    return None


# ==============================================================================
# SECTION 4: TEST GROUPING AND PRIORITIZATION
# ==============================================================================

def analyze_test_groups(js_snap_file: str, ts_snap_file: str) -> Dict:
    """
    Comprehensive analysis of failing tests grouped by category with prioritization.
    
    Args:
        js_snap_file: Path to JavaScript snapshot file
        ts_snap_file: Path to TypeScript snapshot file
    
    Returns:
        Dictionary containing categorized and prioritized test information
    
    Use Case:
        Used to generate the comprehensive test regression analysis that helps
        developers understand which areas need the most attention and in what order.
    """
    # Constants for main branch baseline
    MAIN_JS_PASSING = 525
    MAIN_JS_TOTAL = 699
    MAIN_TS_PASSING = 277
    MAIN_TS_TOTAL = 573
    
    # Read current failing tests
    js_failing = {}
    ts_failing = {}
    
    # Parse JavaScript failures
    with open(js_snap_file, 'r') as f:
        in_failed = False
        for line in f:
            if line.strip() == '# Failed':
                in_failed = True
            elif in_failed and (line.startswith('| js/') or line.startswith('| jsx/')):
                test, percentage = parse_test_line(line)
                if test:
                    js_failing[test] = percentage
    
    # Parse TypeScript failures
    with open(ts_snap_file, 'r') as f:
        in_failed = False
        for line in f:
            if line.strip() == '# Failed':
                in_failed = True
            elif in_failed and line.startswith('| '):
                test, percentage = parse_test_line(line)
                if test and (test.startswith('jsx/') or test.startswith('typescript/')):
                    ts_failing[test] = percentage
    
    # Group by category
    js_categories = group_tests_by_full_category(js_failing)
    ts_categories = group_tests_by_full_category(ts_failing)
    
    # Calculate current passing counts
    current_js_passing = MAIN_JS_TOTAL - len(js_failing)
    current_ts_passing = MAIN_TS_TOTAL - len(ts_failing)
    
    return {
        'main_baseline': {
            'javascript': f"{MAIN_JS_PASSING}/{MAIN_JS_TOTAL} ({MAIN_JS_PASSING/MAIN_JS_TOTAL*100:.2f}%)",
            'typescript': f"{MAIN_TS_PASSING}/{MAIN_TS_TOTAL} ({MAIN_TS_PASSING/MAIN_TS_TOTAL*100:.2f}%)"
        },
        'current_status': {
            'javascript': f"{current_js_passing}/{MAIN_JS_TOTAL} ({current_js_passing/MAIN_JS_TOTAL*100:.2f}%)",
            'typescript': f"{current_ts_passing}/{MAIN_TS_TOTAL} ({current_ts_passing/MAIN_TS_TOTAL*100:.2f}%)",
            'js_needed': MAIN_JS_PASSING - current_js_passing,
            'ts_needed': MAIN_TS_PASSING - current_ts_passing
        },
        'js_categories': js_categories,
        'ts_categories': ts_categories,
        'js_failing_count': len(js_failing),
        'ts_failing_count': len(ts_failing)
    }


def group_tests_by_full_category(tests: Dict[str, float]) -> Dict[str, List[Tuple[str, float]]]:
    """
    Group tests by their full category path (e.g., "js/arrows" not just "arrows").
    
    Args:
        tests: Dictionary of test paths to match percentages
    
    Returns:
        Dictionary mapping full category paths to lists of (test, percentage) tuples
    
    Use Case:
        Provides more detailed categorization than simple grouping, preserving
        the distinction between js/comments and typescript/comments for example.
    """
    categories = defaultdict(list)
    for test, percentage in tests.items():
        parts = test.split('/')
        if len(parts) >= 2:
            # Use first two parts as category (e.g., "js/arrows")
            category = '/'.join(parts[:2])
            categories[category].append((test, percentage))
    return dict(categories)


# ==============================================================================
# SECTION 5: INVESTIGATION HELPERS
# ==============================================================================

def generate_investigation_plan(analysis: Dict) -> List[Dict]:
    """
    Generate a prioritized investigation plan based on test analysis.
    
    Args:
        analysis: Output from analyze_test_groups()
    
    Returns:
        List of investigation tasks with priority, description, and action items
    
    Use Case:
        Creates actionable investigation tasks for developers, helping them
        know exactly what to look at and in what order.
    """
    tasks = []
    
    # Define high-priority categories
    priority_categories = {
        'js/ternaries': ('HIGH', 'Ternary operator formatting issues', 
                        'crates/oxc_formatter/src/format/conditional_expression.rs'),
        'js/test-declarations': ('MEDIUM', 'Test framework declarations (Jest, Angular)',
                                'crates/oxc_formatter/src/write/call_arguments.rs'),
        'js/comments': ('MEDIUM', 'Comment placement and preservation',
                       'crates/oxc_formatter/src/comments.rs'),
        'jsx/jsx': ('LOW', 'JSX expression formatting',
                   'crates/oxc_formatter/src/format/jsx.rs'),
    }
    
    js_cats = analysis.get('js_categories', {})
    
    for category, (priority, description, file_path) in priority_categories.items():
        if category in js_cats:
            tests = js_cats[category]
            task = {
                'category': category,
                'priority': priority,
                'test_count': len(tests),
                'description': description,
                'files': [file_path],
                'worst_tests': sorted(tests, key=lambda x: x[1])[:3],
                'avg_match': sum(t[1] for t in tests) / len(tests) if tests else 0
            }
            tasks.append(task)
    
    # Sort by priority
    priority_order = {'HIGH': 0, 'MEDIUM': 1, 'LOW': 2}
    tasks.sort(key=lambda x: priority_order.get(x['priority'], 3))
    
    return tasks


# ==============================================================================
# SECTION 6: QUICK WIN IDENTIFICATION
# ==============================================================================

def find_quick_wins(failing_tests: Dict[str, float], threshold: float = 85.0) -> List[Tuple[str, float]]:
    """
    Identify tests that are close to passing and might be quick to fix.
    
    Args:
        failing_tests: Dictionary of failing test paths to match percentages
        threshold: Minimum match percentage to consider a quick win
    
    Returns:
        List of (test_path, match_percentage) for tests above threshold
    
    Use Case:
        Helps identify tests that might need only minor tweaks to pass,
        allowing developers to quickly improve the overall pass rate.
    """
    quick_wins = []
    for test, percentage in failing_tests.items():
        if percentage >= threshold:
            quick_wins.append((test, percentage))
    
    # Sort by match percentage (highest first - easiest to fix)
    quick_wins.sort(key=lambda x: x[1], reverse=True)
    
    return quick_wins


# ==============================================================================
# SECTION 7: REPORTING AND VISUALIZATION
# ==============================================================================

def generate_progress_report(checkpoints: List[Dict]) -> str:
    """
    Generate a markdown progress report from a list of checkpoint data.
    
    Args:
        checkpoints: List of dictionaries with commit, javascript, and typescript keys
    
    Returns:
        Markdown-formatted progress table
    
    Use Case:
        Creates the progress tracking table for the conformance fix documentation,
        showing improvement over time with commit hashes for reference.
    """
    report = "| Commit | JavaScript | TypeScript | Notes |\n"
    report += "|--------|------------|------------|-------|\n"
    
    for i, checkpoint in enumerate(checkpoints):
        notes = ""
        if i == 0:
            notes = "Baseline"
        elif i == 1:
            notes = "After decorator fix"
        elif i == 2:
            notes = "After Angular wrapper fix"
        
        report += f"| `{checkpoint['commit']}` | {checkpoint['javascript']} | {checkpoint['typescript']} | {notes} |\n"
    
    return report


def print_category_summary(categories: Dict[str, List[Tuple[str, float]]], top_n: int = 10):
    """
    Print a summary of test categories sorted by number of failures.
    
    Args:
        categories: Dictionary mapping categories to test lists
        top_n: Number of top categories to display
    
    Use Case:
        Provides a quick overview of which areas have the most failures,
        helping identify systematic issues affecting multiple tests.
    """
    # Sort categories by number of failing tests
    sorted_cats = sorted(categories.items(), key=lambda x: len(x[1]), reverse=True)
    
    print(f"\nTop {top_n} Categories by Number of Failures:")
    print("-" * 60)
    
    for category, tests in sorted_cats[:top_n]:
        avg_match = sum(t[1] for t in tests) / len(tests) if tests else 0
        print(f"{category:30} {len(tests):3} tests, avg {avg_match:5.1f}% match")
        
        # Show worst 2 tests in category
        worst = sorted(tests, key=lambda x: x[1])[:2]
        for test, pct in worst:
            test_name = test.split('/')[-1]
            print(f"  - {test_name:40} {pct:5.1f}%")


# ==============================================================================
# SECTION 8: MAIN ANALYSIS WORKFLOW
# ==============================================================================

def run_full_analysis():
    """
    Run the complete conformance test analysis workflow.
    
    Use Case:
        This is the main entry point that combines all the analysis functions
        to provide a complete picture of the conformance test status and
        generate actionable recommendations.
    """
    js_snap = '/Users/tylerearls/code/open-source-projects/forks/oxc/tasks/prettier_conformance/snapshots/prettier.js.snap.md'
    ts_snap = '/Users/tylerearls/code/open-source-projects/forks/oxc/tasks/prettier_conformance/snapshots/prettier.ts.snap.md'
    
    print("=" * 70)
    print("OXC FORMATTER CONFORMANCE ANALYSIS")
    print("=" * 70)
    
    # Run comprehensive analysis
    analysis = analyze_test_groups(js_snap, ts_snap)
    
    # Print status
    print(f"\n📊 Current Status:")
    print(f"  JavaScript: {analysis['current_status']['javascript']}")
    print(f"    Need {analysis['current_status']['js_needed']} more tests to match main")
    print(f"  TypeScript: {analysis['current_status']['typescript']}")
    print(f"    Need {analysis['current_status']['ts_needed']} more tests to match main")
    
    # Show category summary
    print_category_summary(analysis['js_categories'])
    
    # Generate investigation plan
    tasks = generate_investigation_plan(analysis)
    
    print("\n" + "=" * 70)
    print("PRIORITIZED INVESTIGATION PLAN")
    print("=" * 70)
    
    for i, task in enumerate(tasks, 1):
        print(f"\n{i}. [{task['priority']}] {task['category']} ({task['test_count']} tests)")
        print(f"   Average match: {task['avg_match']:.1f}%")
        print(f"   Files to check: {', '.join(task['files'])}")
        print(f"   Worst tests:")
        for test, pct in task['worst_tests']:
            print(f"     - {test.split('/')[-1]}: {pct:.1f}%")
    
    # Find quick wins
    all_failing = {}
    all_failing.update(analysis['js_categories'])
    
    flat_tests = {}
    for tests in analysis['js_categories'].values():
        for test, pct in tests:
            flat_tests[test] = pct
    
    quick_wins = find_quick_wins(flat_tests, 90.0)
    
    if quick_wins:
        print("\n" + "=" * 70)
        print("QUICK WINS (>90% match)")
        print("=" * 70)
        for test, pct in quick_wins[:5]:
            print(f"  {test}: {pct:.1f}%")
    
    return analysis


# ==============================================================================
# USAGE EXAMPLES
# ==============================================================================

if __name__ == "__main__":
    print(__doc__)
    print("\nRunning full conformance analysis...\n")
    
    # Run the complete analysis
    results = run_full_analysis()
    
    print("\n" + "=" * 70)
    print("ANALYSIS COMPLETE")
    print("=" * 70)
    print("\nThis script provides various functions for analyzing Oxc formatter")
    print("conformance test results. Import and use individual functions as needed.")
    print("\nKey functions:")
    print("  - parse_test_line(): Parse individual test result lines")
    print("  - find_regressions(): Compare main vs dev branch results")
    print("  - analyze_test_groups(): Comprehensive test categorization")
    print("  - generate_investigation_plan(): Create prioritized task list")
    print("  - find_quick_wins(): Identify nearly-passing tests")
    print("  - track_progress_after_commit(): Record progress with commit hash")