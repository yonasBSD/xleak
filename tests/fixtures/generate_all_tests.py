#!/usr/bin/env python3
"""
Master script to generate all test files for xleak.
Run: python3 generate_all_tests.py
"""

import subprocess
import sys
import os

def check_dependencies():
    """Check if openpyxl is installed."""
    try:
        import openpyxl
        print("✓ openpyxl is installed\n")
        return True
    except ImportError:
        print("✗ openpyxl is not installed")
        print("  Install with: pip install openpyxl")
        return False

def run_generator(script_name, description):
    """Run a generator script and report status."""
    print(f"{'='*60}")
    print(f"Running {script_name}...")
    print(f"{'='*60}\n")

    try:
        result = subprocess.run(
            [sys.executable, script_name],
            cwd=os.path.dirname(os.path.abspath(__file__)),
            check=True,
            capture_output=False
        )
        print(f"\n✓ {description} generated successfully\n")
        return True
    except subprocess.CalledProcessError as e:
        print(f"\n✗ Failed to generate {description}")
        print(f"  Error: {e}")
        return False

def main():
    """Generate all test files."""
    print("="*60)
    print("xleak Test File Generator")
    print("="*60)
    print()

    # Check dependencies
    if not check_dependencies():
        sys.exit(1)

    # Track success
    all_success = True

    # Generate comprehensive test file
    all_success &= run_generator(
        "generate_test_comprehensive.py",
        "test_comprehensive.xlsx"
    )

    # Generate large test file
    all_success &= run_generator(
        "generate_test_large.py",
        "test_large.xlsx"
    )

    # Generate Excel tables test file
    all_success &= run_generator(
        "generate_test_tables.py",
        "test_tables.xlsx"
    )

    # Summary
    print("="*60)
    if all_success:
        print("✓ All test files generated successfully!")
        print("="*60)
        print()
        print("Generated files:")
        print("  1. test_comprehensive.xlsx - All data types, formulas, edge cases")
        print("  2. test_large.xlsx - 10,000 rows for performance testing")
        print("  3. test_tables.xlsx - Excel Table structures")
        print()
        print("⚠️  IMPORTANT:")
        print("  Open test_comprehensive.xlsx in Excel and save it to cache formula results!")
        print()
        print("Test with:")
        print("  ./target/release/xleak tests/fixtures/test_comprehensive.xlsx -i")
        print("  ./target/release/xleak tests/fixtures/test_large.xlsx -i")
        print("  ./target/release/xleak tests/fixtures/test_tables.xlsx --list-tables")
    else:
        print("✗ Some test files failed to generate")
        print("="*60)
        sys.exit(1)

if __name__ == "__main__":
    main()
