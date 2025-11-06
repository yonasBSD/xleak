use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Direction, Rect, Alignment},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, Paragraph, Clear, Wrap},
    Terminal, Frame,
};
use std::io;
use crate::workbook::SheetData;

/// TUI application state
pub struct TuiState {
    sheet_data: SheetData,
    sheet_name: String,
    should_quit: bool,
    cursor_row: usize,  // Current row (0-indexed in data)
    cursor_col: usize,  // Current column (0-indexed)
    scroll_offset: usize, // Vertical scroll offset
    show_help: bool,    // Help overlay visible
}

impl TuiState {
    pub fn new(sheet_data: SheetData, sheet_name: String) -> Self {
        Self {
            sheet_data,
            sheet_name,
            should_quit: false,
            cursor_row: 0,
            cursor_col: 0,
            scroll_offset: 0,
            show_help: false,
        }
    }

    fn move_up(&mut self) {
        if self.cursor_row > 0 {
            self.cursor_row -= 1;
            // Auto-scroll up if cursor moves above viewport
            if self.cursor_row < self.scroll_offset {
                self.scroll_offset = self.cursor_row;
            }
        }
    }

    fn move_down(&mut self) {
        if self.cursor_row < self.sheet_data.height.saturating_sub(1) {
            self.cursor_row += 1;
            // Auto-scroll down will be handled in render based on viewport height
        }
    }

    /// Update scroll offset to keep cursor visible
    fn update_scroll(&mut self, viewport_height: usize) {
        // Scroll down if cursor is below visible area
        if self.cursor_row >= self.scroll_offset + viewport_height {
            self.scroll_offset = self.cursor_row.saturating_sub(viewport_height - 1);
        }
        // Scroll up if cursor is above visible area
        if self.cursor_row < self.scroll_offset {
            self.scroll_offset = self.cursor_row;
        }
    }

    fn move_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.cursor_col < self.sheet_data.width.saturating_sub(1) {
            self.cursor_col += 1;
        }
    }

    fn move_to_start_of_row(&mut self) {
        self.cursor_col = 0;
    }

    fn move_to_end_of_row(&mut self) {
        self.cursor_col = self.sheet_data.width.saturating_sub(1);
    }

    fn page_up(&mut self, page_size: usize) {
        self.cursor_row = self.cursor_row.saturating_sub(page_size);
    }

    fn page_down(&mut self, page_size: usize) {
        self.cursor_row = (self.cursor_row + page_size).min(self.sheet_data.height.saturating_sub(1));
    }

    fn move_to_top(&mut self) {
        self.cursor_row = 0;
    }

    fn move_to_bottom(&mut self) {
        self.cursor_row = self.sheet_data.height.saturating_sub(1);
    }

    /// Convert column index to Excel-style letter(s)
    fn col_to_letter(&self, col: usize) -> String {
        let mut result = String::new();
        let mut n = col + 1;
        while n > 0 {
            n -= 1;
            result.push((b'A' + (n % 26) as u8) as char);
            n /= 26;
        }
        result.chars().rev().collect()
    }

    /// Get current cell address (e.g., "B7")
    fn current_cell_address(&self) -> String {
        format!("{}{}", self.col_to_letter(self.cursor_col), self.cursor_row + 1)
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event {
            use crossterm::event::KeyModifiers;

            // If help is showing, most keys should just close it
            if self.show_help {
                match code {
                    KeyCode::Char('?') | KeyCode::Esc | KeyCode::Char('q') => {
                        self.show_help = false;
                    }
                    _ => {}
                }
                return;
            }

            // Normal navigation
            match code {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                }
                KeyCode::Char('?') => {
                    self.show_help = true;
                }
                KeyCode::Esc => {
                    self.should_quit = true;
                }
                KeyCode::Up => self.move_up(),
                KeyCode::Down => self.move_down(),
                KeyCode::Left => self.move_left(),
                KeyCode::Right => self.move_right(),
                KeyCode::Home => {
                    if modifiers.contains(KeyModifiers::CONTROL) {
                        self.move_to_top();
                    } else {
                        self.move_to_start_of_row();
                    }
                }
                KeyCode::End => {
                    if modifiers.contains(KeyModifiers::CONTROL) {
                        self.move_to_bottom();
                    } else {
                        self.move_to_end_of_row();
                    }
                }
                KeyCode::PageUp => self.page_up(10),
                KeyCode::PageDown => self.page_down(10),
                _ => {}
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),      // Main content
                Constraint::Length(3),   // Status bar
            ])
            .split(frame.area());

        // Calculate visible viewport
        let table_height = chunks[0].height.saturating_sub(3) as usize; // Account for borders and header

        // Update scroll to keep cursor visible
        self.update_scroll(table_height);

        let visible_start = self.scroll_offset;

        // Build table rows with highlighting
        let header_cells: Vec<Cell> = self.sheet_data.headers
            .iter()
            .enumerate()
            .map(|(col_idx, h)| {
                let style = if col_idx == self.cursor_col {
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                };
                Cell::from(h.as_str()).style(style)
            })
            .collect();

        let header = Row::new(header_cells).height(1);

        let data_rows: Vec<Row> = self.sheet_data.rows
            .iter()
            .enumerate()
            .skip(visible_start)
            .take(table_height)
            .map(|(row_idx, row)| {
                let cells: Vec<Cell> = row.iter()
                    .enumerate()
                    .map(|(col_idx, cell)| {
                        let mut style = Style::default();
                        // Highlight current cell
                        if row_idx == self.cursor_row && col_idx == self.cursor_col {
                            style = style.bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD);
                        }
                        // Highlight current row
                        else if row_idx == self.cursor_row {
                            style = style.bg(Color::DarkGray);
                        }
                        // Highlight current column
                        else if col_idx == self.cursor_col {
                            style = style.fg(Color::Cyan);
                        }
                        Cell::from(cell.to_string()).style(style)
                    })
                    .collect();
                Row::new(cells).height(1)
            })
            .collect();

        // Calculate column widths
        let col_widths: Vec<Constraint> = self.sheet_data.headers
            .iter()
            .map(|_| Constraint::Percentage((100 / self.sheet_data.width.max(1)) as u16))
            .collect();

        let table = Table::new(data_rows, col_widths)
            .header(header)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", self.sheet_name)));

        frame.render_widget(table, chunks[0]);

        // Status bar with current cell info
        let current_cell_value = self.sheet_data.rows
            .get(self.cursor_row)
            .and_then(|row| row.get(self.cursor_col))
            .map(|v| v.to_string())
            .unwrap_or_default();

        let status_text = format!(
            " {} | {} rows × {} columns | ?:help q:quit ",
            self.current_cell_address(),
            self.sheet_data.height,
            self.sheet_data.width
        );

        let status = Paragraph::new(status_text)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title(format!(" {} ", current_cell_value)));

        frame.render_widget(status, chunks[1]);

        // Render help overlay if visible
        if self.show_help {
            self.render_help(frame);
        }
    }

    fn render_help(&self, frame: &mut Frame) {
        let help_text = vec![
            "xleak - Excel File Viewer - Keyboard Shortcuts",
            "",
            "NAVIGATION",
            "  ↑ ↓ ← →           Move cursor",
            "  Page Up/Down      Scroll 10 rows",
            "  Home / End        Jump to start/end of row",
            "  Ctrl+Home         Jump to first row",
            "  Ctrl+End          Jump to last row",
            "",
            "GENERAL",
            "  ?                 Toggle this help",
            "  q / Esc           Quit",
            "",
            "VISUAL CUES",
            "  Blue background   Current cell",
            "  Dark gray bg      Current row",
            "  Cyan text         Current column",
            "",
            "Press any key to close help...",
        ];

        // Calculate popup size (centered, 60% width, auto height)
        let area = frame.area();
        let popup_width = (area.width as f32 * 0.6).min(70.0) as u16;
        let popup_height = (help_text.len() + 4) as u16; // +4 for borders and padding

        let popup_area = Rect {
            x: (area.width.saturating_sub(popup_width)) / 2,
            y: (area.height.saturating_sub(popup_height)) / 2,
            width: popup_width,
            height: popup_height,
        };

        // Clear the area behind the popup
        frame.render_widget(Clear, popup_area);

        // Create help content
        let help_paragraph = Paragraph::new(help_text.join("\n"))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
                    .title(" Help ")
                    .title_alignment(Alignment::Center)
            )
            .wrap(Wrap { trim: false });

        frame.render_widget(help_paragraph, popup_area);
    }
}

/// Run the TUI application
pub fn run_tui(sheet_data: SheetData, sheet_name: &str) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = TuiState::new(sheet_data, sheet_name.to_string());

    // Main event loop
    let res = run_event_loop(&mut terminal, &mut app);

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_event_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut TuiState) -> Result<()> {
    loop {
        // Draw needs mutable access to app for scroll updates
        terminal.draw(|f| {
            app.render(f);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            app.handle_event(event);
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
