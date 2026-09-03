use crate::types::*;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table, Cell as TuiCell},
    text::{Line, Span},
    Terminal,
};
use std::io;

// Fiddling with widths took a while to make the TUI look nice. I'd like a better way to do this
// Likewise with COLUMN_TEMPLATE
const COLUMN_TEMPLATE: [&str; 9] = [
    " | .", " .", " .", "| .", " .", " .", "| .", " .", " . |",
];

const HORIZONTAL_ROW: [&str; 9] = [
    "  --", "--", "---", " --", "--", "---", " --", "--", "---",
];

const WIDTHS: [Constraint; 9] = [
    Constraint::Length(4),
    Constraint::Length(2),
    Constraint::Length(3),
    Constraint::Length(3),
    Constraint::Length(2),
    Constraint::Length(3),
    Constraint::Length(3),
    Constraint::Length(2),
    Constraint::Length(4),
];

pub struct Parser {
    selected_row: usize, // 0..9
    selected_col: usize,
    board: Board
}

impl Parser {
    pub fn new() -> Self {
        Self {
            selected_row: 0,
            selected_col: 0,
            board: [[Cell::Empty ; 9] ; 9]
        }
    }

    fn make_cell_at(&self, row: usize, col: usize) -> (String, String, String) {
        
        let (prefix, suffix) = COLUMN_TEMPLATE[col].split_once('.').unwrap();
        let value = if let Cell::Value(num) = self.board[row][col] {
            String::from(num.to_string())
        } else { 
            String::from('.')
        };
        (String::from(prefix), value, String::from(suffix))
    }

    fn place_digit_in_board(&mut self, digit: i16) {
        let row = self.selected_row;
        let col = self.selected_col;
        self.board[row][col] = Cell::Value(digit);
    }

    fn clear_digit_from_board(&mut self) {
        let row = self.selected_row;
        let col = self.selected_col;
        self.board[row][col] = Cell::Empty;
    }

    fn run_app<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<Option<Board>> {
        let _ = terminal.clear();
        loop {
            // Draw the frame
            terminal.draw(|f| {
                let size = f.area(); 

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(7),  // Top block height (5 instructions, 2 border rows)
                        Constraint::Length(15), // Input Block (9 table rows, 4 separator rows, 2 rows for border)
                        Constraint::Min(0)      // Fill
                    ])
                    .split(size);

                let title_block = Paragraph::new("   Arrow keys to move cursor\n   Press a number to input a value at the cursor\n   Press Delete to clear a value\n   Press Enter to finish inputting and solve\n   Press 'q' or `esc` to quit safely.")
                    .block(Block::default().title(" Sudoku Puzzle Input ").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White));
                
                let mut rows = Vec::new();
                for row in 0..9 {
                    if row % 3 == 0 {
                        rows.push(Row::new(HORIZONTAL_ROW.clone()));
                    }
                    let mut cells = Vec::new();
                    for col in 0..9 {
                        let (prefix, value, suffix) = self.make_cell_at(row, col);
                        let value_style =
                            if row == self.selected_row
                                && col == self.selected_col
                            {
                                Style::default()
                                    .bg(Color::White)
                                    .fg(Color::Black)
                            } else {
                                Style::default()
                            };

                        let cell = TuiCell::from(Line::from(vec![
                            Span::raw(prefix),
                            Span::styled(value, value_style),
                            Span::raw(suffix),
                        ]));
                        cells.push(cell);
                    }
                    rows.push(Row::new(cells));
                }
                rows.push(Row::new(HORIZONTAL_ROW.clone()));

                let entry_field = Table::new(rows, WIDTHS)
                    .column_spacing(0)
                    .block(
                        Block::default()
                            .title(" Board ")
                            .borders(Borders::ALL)
                    );

                // Render components inside specific sections
                f.render_widget(title_block, chunks[0]);
                f.render_widget(entry_field, chunks[1]);
            })?;

            // 4. Handle events/input
            if event::poll(std::time::Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind != KeyEventKind::Press {
                        continue; 
                    }
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
                        KeyCode::Up => {
                            self.selected_row = self.selected_row.saturating_sub(1)
                        }
                        KeyCode::Down => {
                            self.selected_row = std::cmp::min(self.selected_row + 1, 8);
                        }
                        KeyCode::Left => {
                            self.selected_col = self.selected_col.saturating_sub(1)
                        }
                        KeyCode::Right => {
                            self.selected_col = std::cmp::min(self.selected_col + 1, 8);
                        }
                        KeyCode::Char(c) if c.is_ascii_digit() => self.place_digit_in_board(c.to_digit(10).unwrap() as i16),
                        KeyCode::Backspace => self.clear_digit_from_board(),
                        KeyCode::Enter => {
                            // Assume the board is well formed. Implement validity checks later
                            return Ok(Some(self.board))
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn parse (&mut self) -> Result<Option<Board>, io::Error> {
        // 1. Setup the terminal
        enable_raw_mode()?; // Capture key presses instantly without needing Enter
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // 2. Application Loop
        let result = self.run_app(&mut terminal);

        // 3. resulttore the terminal state back to normal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }
}