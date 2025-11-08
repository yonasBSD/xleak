# Test Fixtures

This directory contains test data and scripts for generating test Excel files.

## Test Files

- `test_data.xlsx` - Main test file with 3 sheets (Sales, Employees, Metrics) containing formulas
- `large_test_10000.xlsx` - Large file with 10,000 rows for testing lazy loading
- `comprehensive_test.xlsx` - Comprehensive test file with various data types
- `test_copy.xlsx` - Additional test file

## Generator Scripts

- `generate_test_data.py` - Generates `test_data.xlsx` with formulas (SUM, AVERAGE, IF)
- `generate_large_test_data.py` - Generates large test files with configurable row counts
- `create_comprehensive_test.py` - Generates comprehensive test file with various data types

## Usage

Generate test files by running the Python scripts:

```bash
# Generate main test file
python3 tests/fixtures/generate_test_data.py

# Generate large test file (default 10,000 rows)
python3 tests/fixtures/generate_large_test_data.py

# Generate with custom row count
python3 tests/fixtures/generate_large_test_data.py 50000
```

**Note:** After generating files with formulas, open them in Excel and save to cache formula results.
