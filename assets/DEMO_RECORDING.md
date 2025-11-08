# Creating a Demo GIF for xleak

## Recommended Tools

### Option 1: VHS (Recommended for automation)
[VHS](https://github.com/charmbracelet/vhs) lets you script terminal recordings.

```bash
# Install
brew install vhs

# Create a tape file (see demo.tape below)
vhs demo.tape
```

### Option 2: Asciinema + agg
Record with asciinema, convert to GIF with agg.

```bash
# Install
brew install asciinema
cargo install --git https://github.com/asciinema/agg

# Record
asciinema rec demo.cast

# Convert to GIF
agg demo.cast demo.gif
```

### Option 3: ttystudio
Lightweight terminal recorder.

```bash
npm install -g ttystudio
ttystudio demo.gif
```

## Demo Script

Here's a suggested demo flow showcasing xleak's key features:

1. **Launch interactive mode**
   ```bash
   xleak test_data.xlsx -i
   ```

2. **Navigate cells** (arrow keys for 2-3 seconds)

3. **Search feature** (`/`)
   - Type: `Engineering`
   - Press Enter
   - Press `n` to jump to next match

4. **Jump to row** (`Ctrl+G`)
   - Type: `25`
   - Press Enter

5. **View cell details** (Enter)
   - Show a cell with a formula

6. **Copy cell** (`c`)
   - Show feedback message

7. **Switch sheets** (Tab)
   - Navigate to another sheet

8. **Show help** (`?`)
   - Display help overlay briefly

9. **Quit** (`q`)

## VHS Tape File

Create `demo.tape`:

```tape
Output demo.gif

Set FontSize 14
Set Width 1200
Set Height 700
Set Theme "Dracula"

Type "xleak test_data.xlsx -i"
Enter
Sleep 2s

# Navigate around
Down 3
Right 2
Sleep 1s

# Search
Type "/"
Sleep 500ms
Type "Engineering"
Enter
Sleep 1s
Type "n"
Sleep 1s

# Jump to row
Ctrl+G
Sleep 500ms
Type "15"
Enter
Sleep 1s

# View cell detail
Enter
Sleep 2s
Escape
Sleep 500ms

# Show help
Type "?"
Sleep 3s
Escape
Sleep 500ms

# Quit
Type "q"
Sleep 1s
```

Run with:
```bash
vhs demo.tape
```

## Manual Recording Tips

If recording manually:

1. **Terminal setup:**
   - Clear terminal: `clear`
   - Set appropriate size: ~120x30 is good for readability
   - Use a clean theme (light or dark with good contrast)

2. **File to demo:**
   - Use `test_data.xlsx` (has multiple sheets, formulas, variety of data)

3. **Keep it short:**
   - Aim for 20-30 seconds
   - Focus on 3-4 key features

4. **Features to highlight:**
   - Interactive navigation
   - Search with `/`
   - Formula display (Enter on a formula cell)
   - Jump to row with Ctrl+G
   - Multi-sheet support (Tab)

## Optimizing the GIF

After creating the GIF:

```bash
# Optimize with gifsicle (optional)
brew install gifsicle
gifsicle -O3 --colors 256 demo.gif -o demo-optimized.gif

# Or use online tool: https://ezgif.com/optimize
```

## Adding to README

Once created, add to README:

```markdown
## Demo

![xleak demo](demo.gif)

Or upload to GitHub releases and link:

```markdown
![xleak demo](https://github.com/greenwbm/xleak/releases/download/v0.1.0/demo.gif)
```
