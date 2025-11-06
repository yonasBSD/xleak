# xleak Quick Start Guide 🚀

Get up and running with xleak in under 5 minutes!

## Build the Project

```bash
# Make sure you're in the xleak directory
cd xleak

# Build in release mode for best performance
cargo build --release

# The binary will be at: target/release/xleak
```

## Create Test Data

```bash
# Generate a sample Excel file (requires Python + openpyxl)
pip install openpyxl
python3 generate_test_data.py
```

Or download any `.xlsx` file you have handy!

## Try It Out

### Basic viewing
```bash
# View the test file
cargo run --release -- test_data.xlsx

# Or use the compiled binary directly
./target/release/xleak test_data.xlsx
```

You should see a nicely formatted table with sales data! 📊

### View specific sheets
```bash
# View the Employees sheet
cargo run --release -- test_data.xlsx --sheet Employees

# Or use sheet index (1-based)
cargo run --release -- test_data.xlsx --sheet 2
```

### Limit rows
```bash
# Show only first 3 rows
cargo run --release -- test_data.xlsx -n 3
```

### Export data
```bash
# Export to CSV
cargo run --release -- test_data.xlsx --export csv > sales.csv

# Export to JSON
cargo run --release -- test_data.xlsx --sheet Metrics --export json > metrics.json

# Export as text (tab-separated)
cargo run --release -- test_data.xlsx --export text > data.txt
```

## Install Globally

Once you're happy with it, install xleak system-wide:

```bash
cargo install --path .

# Now you can use it from anywhere!
xleak ~/Documents/report.xlsx
```

## Common Issues

### "File not found"
- Make sure the file path is correct
- Use quotes if the filename has spaces: `xleak "My Report.xlsx"`

### "No sheets found"
- The Excel file might be corrupted
- Try opening it in Excel/LibreOffice first

### "Sheet 'X' not found"
- Use `xleak file.xlsx` (no sheet argument) to see all available sheets
- Sheet names are case-sensitive

## Next Steps

1. Try xleak on your real Excel files
2. Experiment with different export formats
3. Check out the README for more features
4. Star the repo if you find it useful! ⭐

## Getting Help

```bash
# See all available options
xleak --help

# Check version
xleak --version
```

Happy spreadsheeting! 🎉
