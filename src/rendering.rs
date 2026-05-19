use crate::logic::{Board, CurrentPiece, GameState, BOARD_HEIGHT, BOARD_WIDTH};
use bevy::prelude::*;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{self, Stdout};

#[derive(Resource)]
pub struct TuiTerminal {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

pub fn setup_terminal(mut commands: Commands) {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
    terminal.clear().unwrap();

    commands.insert_resource(TuiTerminal { terminal });
}

pub fn restore_terminal(mut exit_events: EventReader<bevy::app::AppExit>) {
    for _ in exit_events.read() {
        crossterm::terminal::disable_raw_mode().unwrap();
        crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();
    }
}

pub fn render_system(
    mut tui: ResMut<TuiTerminal>,
    board: Res<Board>,
    piece_query: Query<&CurrentPiece>,
    game_state: Res<GameState>,
    timer: Res<crate::logic::GravityTimer>,
) {
    tui.terminal
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(BOARD_WIDTH as u16 * 2 + 2),
                    Constraint::Min(20),
                ])
                .split(f.size());

            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(BOARD_HEIGHT as u16 + 2),
                    Constraint::Min(0),
                ])
                .split(chunks[0]);

            let mut board_display = vec![vec![' '; BOARD_WIDTH]; BOARD_HEIGHT];

            // Draw locked pieces
            for y in 0..BOARD_HEIGHT {
                for x in 0..BOARD_WIDTH {
                    if board.grid[y][x].is_some() {
                        board_display[y][x] = '#';
                    }
                }
            }

            // Draw current piece
            if let Ok(piece) = piece_query.get_single() {
                for (dx, dy) in piece.piece_type.coordinates(piece.rotation) {
                    let nx = piece.x + dx;
                    let ny = piece.y + dy;
                    if nx >= 0 && nx < BOARD_WIDTH as i32 && ny >= 0 && ny < BOARD_HEIGHT as i32 {
                        board_display[ny as usize][nx as usize] = '@';
                    }
                }
            }

            let mut content = String::new();
            for y in (0..BOARD_HEIGHT).rev() {
                for x in 0..BOARD_WIDTH {
                    content.push(board_display[y][x]);
                    content.push(' ');
                }
                content.push('\n');
            }

            let block = Block::default()
                .title("Tetris")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow));

            let paragraph = Paragraph::new(content).block(block);
            f.render_widget(paragraph, vertical_chunks[0]);

            // Draw score
            let info_content = format!(
                "Score: {}\nLines: {}\nSpeed: {:.2}s\n\n[Q]uit\n[Down] Drop\n[Up/Z] Rotate",
                game_state.score,
                game_state.lines,
                timer.0.duration().as_secs_f32()
            );
            let info_block = Block::default().title("Info").borders(Borders::ALL);
            let info_paragraph = Paragraph::new(info_content).block(info_block);
            f.render_widget(info_paragraph, chunks[1]);
        })
        .unwrap();
}
