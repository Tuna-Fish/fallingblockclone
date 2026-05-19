use crate::logic::{
    AppMode, Board, CurrentName, CurrentPiece, GameState, HighScores, BOARD_HEIGHT, BOARD_WIDTH,
};
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
    app_mode: Res<AppMode>,
    high_scores: Res<HighScores>,
    current_name: Res<CurrentName>,
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

            match *app_mode {
                AppMode::Playing => {
                    let vertical_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(BOARD_HEIGHT as u16 + 2),
                            Constraint::Min(0),
                        ])
                        .split(chunks[0]);

                    let mut board_display = vec![vec![' '; BOARD_WIDTH]; BOARD_HEIGHT];
                    for y in 0..BOARD_HEIGHT {
                        for x in 0..BOARD_WIDTH {
                            if board.grid[y][x].is_some() {
                                board_display[y][x] = '#';
                            }
                        }
                    }
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
                    f.render_widget(Paragraph::new(content).block(block), vertical_chunks[0]);

                    let info_content = format!(
                        "Score: {}\nLines: {}\nSpeed: {:.2}s\n\n[Q]uit\n[Down] Drop\n[Up/Z] Rotate",
                        game_state.score,
                        game_state.lines,
                        timer.0.duration().as_secs_f32()
                    );
                    f.render_widget(Paragraph::new(info_content).block(Block::default().title("Info").borders(Borders::ALL)), chunks[1]);
                }
                AppMode::Paused => {
                    let content = "\n\n   PAUSED\n\nPress any Arrow key to Resume";
                    f.render_widget(
                        Paragraph::new(content).block(Block::default().title("Pause").borders(Borders::ALL)),
                        f.size(),
                    );
                }
                AppMode::HighScore => {
                    let mut content = String::from("--- HIGH SCORES ---\n\n");
                    for (i, (name, score)) in high_scores.0.iter().enumerate() {
                        content.push_str(&format!("{}. {:<10} {}\n", i + 1, name, score));
                    }
                    content.push_str("\n\nPress any Arrow key or Enter to Start");
                    content.push_str(&format!("\n\nLast Score: {}", game_state.score));
                    content.push_str("\n[Q]uit");

                    f.render_widget(
                        Paragraph::new(content).block(Block::default().title("High Scores").borders(Borders::ALL)),
                        f.size(),
                    );
                }
                AppMode::Naming => {
                    let content = format!(
                        "CONGRATULATIONS!\nYour score: {}\n\nEnter your name: {}\n\n[Enter] to submit",
                        game_state.score, current_name.0
                    );
                    f.render_widget(
                        Paragraph::new(content).block(Block::default().title("New High Score").borders(Borders::ALL)),
                        f.size(),
                    );
                }
            }
        })
        .unwrap();
}
