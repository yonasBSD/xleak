Output assets/demo.gif

Set FontSize 18
Set Width 1200
Set Height 700
Set Theme "Dracula"
Set TypingSpeed 100ms
Set PlaybackSpeed 1.0

# Launch xleak in interactive mode
Type "xleak tests/fixtures/test_data.xlsx -i"
Enter
Sleep 3s

# Navigate around the spreadsheet
Down 2
Sleep 500ms
Right 3
Sleep 500ms
Down 2
Sleep 1s

# Search for "Widgets"
Type "/"
Sleep 300ms
Type "Widgets"
Sleep 500ms
Enter
Sleep 1.5s

# Jump to next match
Type "n"
Sleep 1.5s

# Clear search and jump to a specific row
Escape
Sleep 500ms
Ctrl+G
Sleep 300ms
Type "15"
Sleep 300ms
Enter
Sleep 1s

# View cell details to show formula
Enter
Sleep 2.5s
Escape
Sleep 800ms

# Switch to next sheet
Tab
Sleep 2s

# Copy a cell
Type "c"
Sleep 1.5s

# Show help
Type "?"
Sleep 4s
Escape
Sleep 500ms

# Quit
Type "q"
Sleep 500ms
