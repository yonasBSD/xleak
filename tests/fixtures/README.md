# Test Fixtures

This directory contains test data and generator scripts for xleak.

## Test files

### Core testing
- **test_comprehensive.xlsx** - Primary test file with all data types, formulas, international characters, multi-line cells, date edge cases
  - **DataTypes** sheet - All basic types (strings, numbers, dates, booleans, errors, empty cells)
  - **Formulas** sheet - SUM, AVERAGE, MIN, MAX, IF, ROUND, etc.
  - **Internationalization** sheet - UTF-8 characters: German (ä, ö, ü, ß), Turkish (İ, ı, ş), Chinese (简体中文), Japanese (日本語) (Issue #11)
  - **MultilineCells** sheet - Cells with 1, 5, 10, 20, 50, 100 lines (Issue #16)
  - **DateEdgeCases** sheet - 1900 leap year bug, Feb 29 1900, modern dates, Issue #25 test cases
  - **EdgeCases** sheet - Long strings, whitespace variations, special characters

### Performance & advanced
- **test_large.xlsx** - 10,000 rows for lazy loading and performance testing
  - **LargeData** sheet - 10,000 rows × 12 columns
  - **SmallData** sheet - 50 rows for multi-sheet navigation testing

- **test_tables.xlsx** - Excel Table structures (Issue #18)
  - **ProductsTable** - Products table with 10 items
  - **SalesTable** - Sales table with order data
  - **EmployeesTable** - Employees table with 15 records
  - **UnformattedData** - Regular range (non-table) for comparison

## Generator scripts

- **generate_all_tests.py** - Master script to regenerate all test files
- **generate_test_comprehensive.py** - Creates test_comprehensive.xlsx
- **generate_test_large.py** - Creates test_large.xlsx (configurable row count)
- **generate_test_tables.py** - Creates test_tables.xlsx

## Usage

### Generate all test files

```bash
# From project root with .venv activated
source .venv/bin/activate
cd tests/fixtures
python generate_all_tests.py
```

### Generate individual files

```bash
# Comprehensive test file
python generate_test_comprehensive.py

# Large dataset (default 10,000 rows)
python generate_test_large.py

# Custom size large dataset
python generate_test_large.py 50000

# Excel tables
python generate_test_tables.py
```

### Important: formula caching

After generating test_comprehensive.xlsx, **open it in Excel and save** to cache formula results. Formulas will show as `#NAME?` in xleak until Excel calculates them.

## Testing with xleak

```bash
# Comprehensive test (interactive TUI)
./target/release/xleak tests/fixtures/test_comprehensive.xlsx -i

# View specific sheets
./target/release/xleak tests/fixtures/test_comprehensive.xlsx --sheet DateEdgeCases
./target/release/xleak tests/fixtures/test_comprehensive.xlsx --sheet MultilineCells

# Test large file with lazy loading
./target/release/xleak tests/fixtures/test_large.xlsx -i
./target/release/xleak tests/fixtures/test_large.xlsx --sheet LargeData -n 20

# Test Excel tables
./target/release/xleak tests/fixtures/test_tables.xlsx --list-tables
./target/release/xleak tests/fixtures/test_tables.xlsx --table Products -i

# Test date fix (Issue #25)
./target/release/xleak tests/fixtures/test_comprehensive.xlsx --sheet DateEdgeCases
# Row 9-10 should show 2025-11-19 (not 2025-11-18)
```

## Deprecated files

The following files have been consolidated into the new test structure and moved to `deprecated/`:
- test_data.xlsx → merged into test_comprehensive.xlsx
- comprehensive_test.xlsx → replaced by test_comprehensive.xlsx
- multiline_test.xlsx → merged as MultilineCells sheet
- persons.xlsx → merged into DateEdgeCases sheet
- test_copy.xlsx → merged into EdgeCases sheet
- large_test_10000.xlsx → replaced by test_large.xlsx

Old generator scripts:
- generate_test_data.py → replaced by generate_test_comprehensive.py
- generate_large_test_data.py → replaced by generate_test_large.py
- create_comprehensive_test.py → replaced by generate_test_comprehensive.py
