use bevy::prelude::*;
use rand::seq::SliceRandom;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl PieceType {
    pub const ALL: [PieceType; 7] = [
        PieceType::I,
        PieceType::O,
        PieceType::T,
        PieceType::S,
        PieceType::Z,
        PieceType::J,
        PieceType::L,
    ];

    pub fn coordinates(&self, rotation: u8) -> [(i32, i32); 4] {
        match self {
            PieceType::I => match rotation % 2 {
                0 => [(0, 1), (1, 1), (2, 1), (3, 1)],
                _ => [(2, 0), (2, 1), (2, 2), (2, 3)],
            },
            PieceType::O => [(1, 1), (2, 1), (1, 2), (2, 2)],
            PieceType::T => match rotation % 4 {
                0 => [(1, 1), (0, 1), (2, 1), (1, 2)],
                1 => [(1, 1), (1, 0), (1, 2), (2, 1)],
                2 => [(1, 1), (0, 1), (2, 1), (1, 0)],
                _ => [(1, 1), (1, 0), (1, 2), (0, 1)],
            },
            PieceType::S => match rotation % 2 {
                0 => [(1, 1), (2, 1), (0, 2), (1, 2)],
                _ => [(1, 1), (1, 2), (2, 2), (2, 3)],
            },
            PieceType::Z => match rotation % 2 {
                0 => [(0, 1), (1, 1), (1, 2), (2, 2)],
                _ => [(2, 1), (2, 2), (1, 2), (1, 3)],
            },
            PieceType::J => match rotation % 4 {
                0 => [(0, 1), (1, 1), (2, 1), (2, 2)],
                1 => [(1, 0), (1, 1), (1, 2), (0, 2)],
                2 => [(0, 0), (0, 1), (1, 1), (2, 1)],
                _ => [(1, 0), (2, 0), (1, 1), (1, 2)],
            },
            PieceType::L => match rotation % 4 {
                0 => [(0, 1), (1, 1), (2, 1), (0, 2)],
                1 => [(0, 0), (1, 0), (1, 1), (1, 2)],
                2 => [(2, 0), (0, 1), (1, 1), (2, 1)],
                _ => [(1, 0), (1, 1), (1, 2), (2, 2)],
            },
        }
    }
}

#[derive(Component)]
pub struct CurrentPiece {
    pub piece_type: PieceType,
    pub x: i32,
    pub y: i32,
    pub rotation: u8,
}

#[derive(Resource, Default)]
pub struct GameState {
    pub lines: u32,
    pub score: u32,
}

#[derive(Resource, Default, PartialEq)]
pub enum AppMode {
    #[default]
    HighScore,
    Playing,
    Naming,
}

#[derive(Resource, Default)]
pub struct HighScores(pub Vec<(String, u32)>);

#[derive(Resource, Default)]
pub struct CurrentName(pub String);

pub fn load_scores(_scores: &mut Vec<(String, u32)>) {
    // Stub
}

pub fn save_scores(_scores: &[(String, u32)]) {
    // Stub
}

#[derive(Resource, Default)]
pub struct Board {
    pub grid: [[Option<PieceType>; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn is_colliding(&self, piece_type: PieceType, x: i32, y: i32, rotation: u8) -> bool {
        for (dx, dy) in piece_type.coordinates(rotation) {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || nx >= BOARD_WIDTH as i32 || ny < 0 {
                return true;
            }
            if ny < BOARD_HEIGHT as i32 {
                if self.grid[ny as usize][nx as usize].is_some() {
                    return true;
                }
            }
        }
        false
    }

    pub fn lock_piece(&mut self, piece_type: PieceType, x: i32, y: i32, rotation: u8) {
        for (dx, dy) in piece_type.coordinates(rotation) {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < BOARD_WIDTH as i32 && ny >= 0 && ny < BOARD_HEIGHT as i32 {
                self.grid[ny as usize][nx as usize] = Some(piece_type);
            }
        }
    }

    pub fn clear_lines(&mut self) -> u32 {
        let mut lines_cleared = 0;
        let mut y = 0;
        while y < BOARD_HEIGHT {
            let mut full = true;
            for x in 0..BOARD_WIDTH {
                if self.grid[y][x].is_none() {
                    full = false;
                    break;
                }
            }
            if full {
                lines_cleared += 1;
                for move_y in y..(BOARD_HEIGHT - 1) {
                    self.grid[move_y] = self.grid[move_y + 1];
                }
                self.grid[BOARD_HEIGHT - 1] = [None; BOARD_WIDTH];
            } else {
                y += 1;
            }
        }
        lines_cleared
    }
}

#[derive(Resource)]
pub struct PieceBag {
    pub pieces: Vec<PieceType>,
}

impl Default for PieceBag {
    fn default() -> Self {
        Self { pieces: Vec::new() }
    }
}

#[derive(Resource)]
pub struct GravityTimer(pub Timer);

impl Default for GravityTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

#[derive(Event)]
pub enum GameAction {
    MoveLeft,
    MoveRight,
    MoveDown,
    Rotate,
    GravityStep,
    StartGame,
    SubmitName,
    KeyPressed(char),
    Backspace,
}

pub fn spawn_piece(
    mut commands: Commands,
    mut bag: ResMut<PieceBag>,
    current_piece: Query<Entity, With<CurrentPiece>>,
    board: Res<Board>,
    mut app_mode: ResMut<AppMode>,
    game_state: Res<GameState>,
    high_scores: Res<HighScores>,
) {
    if *app_mode != AppMode::Playing || !current_piece.is_empty() {
        return;
    }

    if bag.pieces.is_empty() {
        let mut new_bag = PieceType::ALL.to_vec();
        let mut rng = rand::thread_rng();
        new_bag.shuffle(&mut rng);
        bag.pieces = new_bag;
    }

    if let Some(piece_type) = bag.pieces.pop() {
        let x = BOARD_WIDTH as i32 / 2 - 2;
        let y = BOARD_HEIGHT as i32 - 4;
        let rotation = 0;
        if board.is_colliding(piece_type, x, y, rotation) {
            if high_scores.0.len() < 10
                || game_state.score > high_scores.0.last().map(|s| s.1).unwrap_or(0)
            {
                *app_mode = AppMode::Naming;
            } else {
                *app_mode = AppMode::HighScore;
            }
            return;
        }
        commands.spawn(CurrentPiece {
            piece_type,
            x,
            y,
            rotation,
        });
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    mut timer: ResMut<GravityTimer>,
    mut actions: EventWriter<GameAction>,
    app_mode: Res<AppMode>,
) {
    if *app_mode == AppMode::Playing && timer.0.tick(time.delta()).just_finished() {
        actions.send(GameAction::GravityStep);
    }
}

pub fn handle_actions(
    mut actions: EventReader<GameAction>,
    mut board: ResMut<Board>,
    mut piece_query: Query<(Entity, &mut CurrentPiece)>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut timer: ResMut<GravityTimer>,
    mut app_mode: ResMut<AppMode>,
    mut high_scores: ResMut<HighScores>,
    mut current_name: ResMut<CurrentName>,
    mut bag: ResMut<PieceBag>,
) {
    for action in actions.read() {
        match app_mode.as_ref() {
            AppMode::HighScore => {
                if let GameAction::StartGame = action {
                    *board = Board::default();
                    *game_state = GameState::default();
                    *timer = GravityTimer::default();
                    *bag = PieceBag::default();
                    *app_mode = AppMode::Playing;
                    load_scores(&mut high_scores.0);
                }
            }
            AppMode::Naming => match action {
                GameAction::KeyPressed(c) => current_name.0.push(*c),
                GameAction::Backspace => {
                    current_name.0.pop();
                }
                GameAction::SubmitName => {
                    high_scores
                        .0
                        .push((current_name.0.clone(), game_state.score));
                    high_scores.0.sort_by(|a, b| b.1.cmp(&a.1));
                    high_scores.0.truncate(10);
                    save_scores(&high_scores.0);
                    current_name.0.clear();
                    *app_mode = AppMode::HighScore;
                }
                _ => {}
            },
            AppMode::Playing => {
                let Ok((entity, mut piece)) = piece_query.get_single_mut() else {
                    continue;
                };
                match action {
                    GameAction::MoveLeft => {
                        if !board.is_colliding(
                            piece.piece_type,
                            piece.x - 1,
                            piece.y,
                            piece.rotation,
                        ) {
                            piece.x -= 1;
                        }
                    }
                    GameAction::MoveRight => {
                        if !board.is_colliding(
                            piece.piece_type,
                            piece.x + 1,
                            piece.y,
                            piece.rotation,
                        ) {
                            piece.x += 1;
                        }
                    }
                    GameAction::MoveDown => {
                        while !board.is_colliding(
                            piece.piece_type,
                            piece.x,
                            piece.y - 1,
                            piece.rotation,
                        ) {
                            piece.y -= 1;
                        }
                        board.lock_piece(piece.piece_type, piece.x, piece.y, piece.rotation);
                        commands.entity(entity).despawn();
                        let cleared = board.clear_lines();
                        if cleared > 0 {
                            let old_lines = game_state.lines;
                            game_state.lines += cleared;
                            game_state.score += cleared * 10;
                            match cleared {
                                2 => game_state.score += 2,
                                3 => game_state.score += 3,
                                4 => game_state.score += 5,
                                _ => {}
                            }
                            if game_state.lines / 10 > old_lines / 10 {
                                let new_duration = timer.0.duration().mul_f32(0.9);
                                timer.0.set_duration(new_duration);
                            }
                        }
                    }
                    GameAction::GravityStep => {
                        if !board.is_colliding(
                            piece.piece_type,
                            piece.x,
                            piece.y - 1,
                            piece.rotation,
                        ) {
                            piece.y -= 1;
                        } else {
                            board.lock_piece(piece.piece_type, piece.x, piece.y, piece.rotation);
                            commands.entity(entity).despawn();
                            let cleared = board.clear_lines();
                            if cleared > 0 {
                                let old_lines = game_state.lines;
                                game_state.lines += cleared;
                                game_state.score += cleared * 10;
                                match cleared {
                                    2 => game_state.score += 2,
                                    3 => game_state.score += 3,
                                    4 => game_state.score += 5,
                                    _ => {}
                                }
                                if game_state.lines / 10 > old_lines / 10 {
                                    let new_duration = timer.0.duration().mul_f32(0.9);
                                    timer.0.set_duration(new_duration);
                                }
                            }
                        }
                    }
                    GameAction::Rotate => {
                        let next_rotation = (piece.rotation + 1) % 4;
                        if !board.is_colliding(piece.piece_type, piece.x, piece.y, next_rotation) {
                            piece.rotation = next_rotation;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
