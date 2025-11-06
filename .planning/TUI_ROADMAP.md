# xleak TUI Implementation Roadmap

This is a comprehensive, incremental roadmap to take xleak from its current CLI state to a fully-featured TUI application. Check off items as we complete them!

---

## Phase 1: Quick Fixes & Display Improvements ✅ COMPLETED

**Goal:** Fix current display issues before building TUI on top of them

### Column Width Management
- [x] Add `--max-width` CLI flag to limit column width (default: 30 chars)
- [x] Implement column truncation with "..." indicator
- [x] Add column wrapping option for text cells
- [x] Improve auto-sizing algorithm in prettytable usage
- [x] Test with very wide columns (100+ char strings)

**Files:** `src/main.rs`, `src/display.rs`

### Better Error Handling
- [x] Handle empty sheets without errors (show friendly message)
- [x] Handle sheets with only headers (no data rows)
- [x] Add file path to all error messages
- [x] List available sheets when sheet not found
- [x] Validate export format before processing

**Files:** `src/main.rs`, `src/workbook.rs`, `src/display.rs`

### Display Polish
- [x] Remove emojis from default display (keep for errors only)
- [x] Simplify header box formatting
- [x] Add totals row count summary at bottom
- [x] Improve number formatting (commas for thousands)
- [x] Better boolean display (true/false vs TRUE/FALSE)

**Files:** `src/display.rs`

---

## Phase 2: TUI Foundation ✅ COMPLETED

**Goal:** Set up basic TUI infrastructure and event loop

### Dependency Setup
- [x] Uncomment `ratatui = "0.29"` in Cargo.toml
- [x] Uncomment `crossterm = "0.28"` in Cargo.toml
- [x] Run `cargo update` to fetch dependencies
- [x] Test compilation with new deps

**Files:** `Cargo.toml`

### TUI Module Creation
- [x] Create `src/tui.rs` module file
- [x] Add `mod tui;` to `src/main.rs`
- [x] Create `TuiState` struct to hold app state
- [x] Create `run_tui()` function with basic loop
- [x] Implement terminal setup (enter/exit alternate screen)
- [x] Add basic event handling (quit on 'q' or Esc)

**Files:** `src/tui.rs` (new), `src/main.rs`

### CLI Integration
- [x] Add `--interactive` / `-i` flag to `Cli` struct
- [x] Add logic to call `run_tui()` when `-i` is set
- [x] Pass sheet data and metadata to TUI
- [x] Test flag works and switches modes

**Files:** `src/main.rs`

### Basic Grid Rendering
- [x] Use ratatui `Table` widget to render sheet
- [x] Display headers in TUI (yellow, bold)
- [x] Display all data rows
- [x] Add status bar at bottom (sheet name, dimensions, quit hint)
- [x] Test with test_data.xlsx

**Files:** `src/tui.rs`

---

## Phase 3: Navigation & Scrolling ✅ COMPLETED

**Goal:** Make TUI interactive with full sheet navigation

### Cell Navigation
- [x] Implement arrow key handling (Up/Down/Left/Right)
- [x] Track current cell position (row, col)
- [x] Highlight current cell with different style (blue background, bold)
- [x] Highlight current row (dark gray background)
- [x] Highlight current column (cyan text)
- [x] Handle edge cases (can't go beyond boundaries)
- [x] Add Home/End key support (jump to start/end of row)

**Files:** `src/tui.rs`

### Scrolling
- [x] Implement vertical scrolling (keep viewport centered on cursor)
- [x] Implement automatic scroll adjustment
- [x] Add Page Up/Page Down support (10 rows at a time)
- [x] Add Ctrl+Home/End (jump to first/last row)
- [x] Handle large sheets efficiently (viewport-based rendering)

**Files:** `src/tui.rs`

### Sheet Switching (Deferred to later)
- [ ] Implement Tab key to switch to next sheet
- [ ] Implement Shift+Tab to switch to previous sheet
- [ ] Show current sheet in status bar
- [ ] Show sheet count in status bar (e.g., "Sheet 2/5")
- [ ] Load new sheet data when switching
- [ ] Reset cursor position on sheet change

**Files:** `src/tui.rs`, `src/workbook.rs`

### Status Bar Enhancements
- [x] Show current cell position (e.g., "B7")
- [x] Show current cell value in status bar title
- [x] Show total rows/columns
- [x] Show current sheet name (in table title)
- [x] Show available keyboard shortcuts
- [x] Excel-style column letters (A, B, C...AA, AB, etc.)

**Files:** `src/tui.rs`

---

## Phase 4: Advanced Features

**Goal:** Add powerful features matching doxx functionality

### Search Functionality
- [ ] Implement `/` key to open search prompt
- [ ] Create search input box widget
- [ ] Implement case-insensitive search across all cells
- [ ] Highlight matching cells
- [ ] Add `n` / `N` keys to jump to next/previous match
- [ ] Show match count (e.g., "3 of 12")
- [ ] Clear search with Esc

**Files:** `src/tui.rs`

### Formula Display
- [ ] Add `f` key toggle for formula mode
- [ ] Update workbook to store formulas (requires calamine changes)
- [ ] Display formula bar above grid
- [ ] Show current cell formula in formula bar
- [ ] Toggle between formula and value display
- [ ] Handle cells without formulas gracefully

**Files:** `src/tui.rs`, `src/workbook.rs`

### Clipboard Support
- [ ] Add `c` key to copy current cell
- [ ] Add `C` (Shift+C) to copy current row
- [ ] Add visual feedback when copying
- [ ] Use `arboard` or `clipboard` crate for cross-platform support
- [ ] Handle copy errors gracefully

**Files:** `src/tui.rs`, `Cargo.toml` (add clipboard crate)

### Cell Detail View
- [ ] Add `Enter` key to show cell detail popup
- [ ] Show full cell contents (no truncation)
- [ ] Show cell type (String, Int, Float, etc.)
- [ ] Show cell formatting info
- [ ] Add ability to close popup (Esc)

**Files:** `src/tui.rs`

### Export from TUI
- [ ] Add `e` key to open export menu
- [ ] Show export format options (CSV, JSON, text)
- [ ] Allow choosing output file path
- [ ] Show export progress for large files
- [ ] Return to TUI after export completes

**Files:** `src/tui.rs`, `src/display.rs`

### Help Screen ✅ COMPLETED
- [x] Add `?` key to show help overlay
- [x] List all keyboard shortcuts
- [x] Group shortcuts by category (Navigation, General, Visual Cues)
- [ ] Add search within help (future - deferred)
- [x] Close help with `?`, Esc, or any key

**Files:** `src/tui.rs`

---

## Phase 5: Polish & Optimization

**Goal:** Make TUI production-ready and delightful

### Visual Polish
- [ ] Add color theme support (light/dark)
- [ ] Implement cell type coloring (numbers blue, strings white, etc.)
- [ ] Add grid lines option (toggle with `g`)
- [ ] Improve header styling (bold, different bg color)
- [ ] Add alternating row colors for readability

**Files:** `src/tui.rs`

### Performance Optimization
- [ ] Lazy load sheets (don't load all into memory)
- [ ] Implement virtual scrolling for huge sheets
- [ ] Profile and optimize hot paths
- [ ] Add progress indicator for large file loading
- [ ] Cache rendered cells

**Files:** `src/tui.rs`, `src/workbook.rs`

### Configuration
- [ ] Create `~/.config/xleak/config.toml` support
- [ ] Allow customizing keybindings
- [ ] Allow setting default theme
- [ ] Allow setting default max-width
- [ ] Add `--config` flag to specify config file

**Files:** `src/tui.rs`, `src/main.rs`, `Cargo.toml` (add toml/serde)

### Mouse Support
- [ ] Enable mouse events in crossterm
- [ ] Allow clicking cells to select
- [ ] Allow scrolling with mouse wheel
- [ ] Allow clicking sheet tabs to switch
- [ ] Add double-click for cell detail view

**Files:** `src/tui.rs`

### Advanced Navigation
- [ ] Add `Ctrl+F` as alternative search trigger
- [ ] Add `Ctrl+G` to go to specific cell (e.g., "B100")
- [ ] Add filter mode to show only matching rows
- [ ] Add sort by column (click header or hotkey)
- [ ] Add freeze panes (keep headers visible)

**Files:** `src/tui.rs`

---

## Phase 6: Testing & Documentation

**Goal:** Ensure quality and usability

### Testing
- [ ] Add unit tests for TUI state management
- [ ] Add integration tests for navigation
- [ ] Test with various file formats (.xlsx, .xls, .ods)
- [ ] Test with edge cases (empty sheets, single cell, huge files)
- [ ] Test on macOS, Linux, Windows

**Files:** `tests/` (new directory)

### Documentation
- [ ] Update README.md with TUI screenshots
- [ ] Update QUICKSTART.md with TUI examples
- [ ] Document all keyboard shortcuts
- [ ] Create GIF demo for README
- [ ] Update AGENTS.md with TUI architecture notes

**Files:** `README.md`, `QUICKSTART.md`, `AGENTS.md`

### Release Preparation
- [ ] Bump version to 1.0.0
- [ ] Create CHANGELOG.md
- [ ] Add CI/CD for releases
- [ ] Publish to crates.io
- [ ] Create GitHub release with binaries

**Files:** `Cargo.toml`, `CHANGELOG.md` (new), `.github/workflows/` (new)

---

## Notes

### Current State (as of 2025-11-05)
- ✅ Basic CLI works with prettytable display
- ✅ Multi-sheet support
- ✅ CSV/JSON/text export
- ✅ Row limiting with `-n` flag
- ✅ Column width limiting with `--max-width` flag (Phase 1 completed!)
- ✅ Professional display formatting (thousand separators, lowercase booleans, clean header)
- ✅ **Proper date rendering** (Excel dates show as YYYY-MM-DD)
- ✅ Excellent error handling (file paths, sheet listings, helpful messages)
- ✅ **Interactive TUI mode with `-i` flag** (Phase 2 completed!)
- ✅ **Full navigation with arrow keys, Page Up/Down, Home/End** (Phase 3 completed!)
- ✅ Cell, row, and column highlighting
- ✅ Automatic viewport scrolling
- ✅ Excel-style cell addresses (A1, B7, etc.)
- ✅ Current cell value display in status bar
- ✅ **Help screen with `?` key** (Phase 4 - Help Screen completed!)
- ❌ No search functionality yet (Phase 4: Search next!)

### Dependencies to Add
- `ratatui` - TUI framework
- `crossterm` - Terminal manipulation
- `arboard` or `clipboard` - Clipboard support (Phase 4)
- `serde` + `serde_json` - Better JSON export (optional)
- `toml` + `serde` - Config file support (Phase 5)

### References
- [ratatui tutorial](https://ratatui.rs/tutorial/)
- [doxx](https://github.com/bgreenwell/doxx) - our inspiration
- [calamine docs](https://docs.rs/calamine/) - Excel parsing

### Testing Strategy
After each phase:
1. Run `cargo fmt && cargo clippy && cargo build --release`
2. Test with `test_data.xlsx`
3. Test with a real-world Excel file
4. Verify no regressions in CLI mode
