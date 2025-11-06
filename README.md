# xleak 📊

> Expose Excel files in your terminal - no Microsoft Excel required!

Inspired by [doxx](https://github.com/bgreenwell/doxx), `xleak` brings Excel spreadsheets to your command line with beautiful rendering and powerful export capabilities.

## Features

- 📊 **Beautiful terminal rendering** with formatted tables
- 🔢 **Smart data type handling** - numbers right-aligned, text left-aligned, booleans centered
- 📑 **Multi-sheet support** - navigate between sheets easily
- 📤 **Multiple export formats** - CSV, JSON, plain text
- ⚡ **Blazing fast** - powered by `calamine`, the fastest Excel parser in Rust
- 🎯 **Multiple formats** - supports `.xlsx`, `.xls`, `.xlsm`, `.xlsb`, `.ods`

## Installation

```bash
# Install from source
git clone <your-repo-url>
cd xleak
cargo install --path .
```

## Usage

### View a spreadsheet
```bash
xleak quarterly-report.xlsx
```

### View a specific sheet
```bash
# By name
xleak report.xlsx --sheet "Q3 Results"

# By index (1-based)
xleak report.xlsx --sheet 2
```

### Limit displayed rows
```bash
# Show only first 20 rows
xleak large-file.xlsx -n 20

# Show all rows
xleak file.xlsx -n 0
```

### Export data
```bash
# Export to CSV
xleak data.xlsx --export csv > output.csv

# Export to JSON
xleak data.xlsx --export json > output.json

# Export as plain text (tab-separated)
xleak data.xlsx --export text > output.txt
```

### Combine options
```bash
# Export specific sheet as CSV
xleak workbook.xlsx --sheet "Sales" --export csv > sales.csv
```

## Examples

```bash
# Quick preview of first sheet
xleak quarterly-report.xlsx

# See specific sheet with limited rows
xleak financial-data.xlsx --sheet "Summary" -n 10

# Export all data from a sheet
xleak survey-results.xlsx --sheet "Responses" --export csv -n 0
```

## Keyboard Shortcuts (Coming Soon)

Interactive TUI mode with:
- Tab/Shift+Tab - Switch between sheets
- Arrow keys - Navigate cells
- `/` - Search
- `c` - Copy to clipboard
- `q` - Quit

## Roadmap

- [x] Basic CLI with table display
- [x] CSV/JSON/text export
- [ ] Interactive TUI with ratatui
- [ ] Search functionality
- [ ] Formula display mode
- [ ] Cell formatting visualization
- [ ] Copy to clipboard
- [ ] Large file streaming

## Comparison to Alternatives

| Tool | Format | Speed | Terminal Native | Interactive |
|------|--------|-------|----------------|-------------|
| **xleak** | ✅ xlsx/xls/ods | ⚡ Fast | ✅ Yes | 🚧 Coming |
| Excel | ✅ xlsx | ❌ Slow startup | ❌ GUI only | ✅ Yes |
| pandas | ✅ Many | ❌ Slow | ❌ Python required | ❌ No |
| csvlook | ❌ CSV only | ✅ Fast | ✅ Yes | ❌ No |

## Built With

- 🦀 **Rust** - for performance and reliability
- 📊 **calamine** - the fastest Excel/ODS parser
- 🎨 **prettytable-rs** - beautiful terminal tables
- 🔧 **clap** - elegant CLI argument parsing

## License

MIT

## Credits

- Inspired by [doxx](https://github.com/bgreenwell/doxx) by bgreenwell
- Powered by [calamine](https://github.com/tafia/calamine)
