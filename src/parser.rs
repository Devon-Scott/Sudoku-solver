use crate::types::*;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Row, Table, TableState},
    Terminal,
};
use std::io;

pub struct Parser {}

impl Parser {
    // fn send_input_to_board(&self) -> Board {

    // }

    fn run_app<B: ratatui::backend::Backend>(&self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            // Draw the frame
            terminal.draw(|f| {
                let size = f.area(); // Grab entire screen space

                // Divide the space into two vertical halves
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(7),  // Top block height (5 instructions, 2 border rows)
                        Constraint::Length(15), // Input Block (9 table rows, 4 separator rows, 2 rows for border)
                        Constraint::Min(0)      // Fill
                    ])
                    .split(size);

                // // Create widgets
                // let title_block = Block::default()
                //     .title(" Sudoku Puzzle Input ")
                //     .borders(Borders::ALL)
                //     .border_style(Style::default().fg(Color::White));

                let title_block = Paragraph::new("   Arrow keys to move cursor\n   Press a number to input a value at the cursor\n   Press Delete to clear a value\n   Press Enter to finish inputting and solve\n   Press 'q' or `esc` to quit safely.")
                    .block(Block::default().title(" Sudoku Puzzle Input ").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White));

                // Fiddling with widths took a while to make the TUI look nice. I'd like a better way to do this
                let number_row = Row::new(vec![
                    " | .", " .", " .", "| .", " .", " .", "| .", " .", " . |",
                ]);

                let horizontal_row = Row::new(vec![
                    "  --", "--", "---", " --", "--", "---", " --", "--", "---",
                ]);

                let mut rows = Vec::new();
                for row in 0..9 {
                    if row % 3 == 0 {
                        rows.push(horizontal_row.clone());
                    }
                    rows.push(number_row.clone());
                }
                rows.push(horizontal_row.clone());

                let widths = [
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

                let entry_field = Table::new(rows, widths)
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
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
            }
        }
    }

    pub fn parse (&self) -> Result<(), io::Error> {
        // 1. Setup the terminal
        enable_raw_mode()?; // Capture key presses instantly without needing Enter
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // 2. Application Loop
        let res = self.run_app(&mut terminal);

        // 3. Restore the terminal state back to normal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("Error: {err:?}");
        }

        Ok(())
    }
}