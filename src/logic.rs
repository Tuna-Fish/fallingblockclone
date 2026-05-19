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
        // Simple rotation logic (simplified for this clone)
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
pub struct Board {
    pub grid: [[Option<PieceType>; BOARD_WIDTH]; BOARD_HEIGHT],
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

#[derive(Event)]
pub enum GameAction {
    MoveLeft,
    MoveRight,
    MoveDown,
    Rotate,
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
}

pub fn handle_actions(
    mut actions: EventReader<GameAction>,
    mut board: ResMut<Board>,
    mut piece_query: Query<(Entity, &mut CurrentPiece)>,
    mut commands: Commands,
) {
    let Ok((entity, mut piece)) = piece_query.get_single_mut() else {
        return;
    };

    for action in actions.read() {
        match action {
            GameAction::MoveLeft => {
                if !board.is_colliding(piece.piece_type, piece.x - 1, piece.y, piece.rotation) {
                    piece.x -= 1;
                }
            }
            GameAction::MoveRight => {
                if !board.is_colliding(piece.piece_type, piece.x + 1, piece.y, piece.rotation) {
                    piece.x += 1;
                }
            }
            GameAction::MoveDown => {
                if !board.is_colliding(piece.piece_type, piece.x, piece.y - 1, piece.rotation) {
                    piece.y -= 1;
                } else {
                    board.lock_piece(piece.piece_type, piece.x, piece.y, piece.rotation);
                    commands.entity(entity).despawn();
                    return; // Stop processing actions if piece is locked
                }
            }
            GameAction::Rotate => {
                let next_rotation = (piece.rotation + 1) % 4;
                if !board.is_colliding(piece.piece_type, piece.x, piece.y, next_rotation) {
                    piece.rotation = next_rotation;
                }
            }
        }
    }
}

pub fn spawn_piece(
    mut commands: Commands,
    mut bag: ResMut<PieceBag>,
    current_piece: Query<Entity, With<CurrentPiece>>,
) {
    if !current_piece.is_empty() {
        return;
    }

    if bag.pieces.is_empty() {
        let mut new_bag = PieceType::ALL.to_vec();
        let mut rng = rand::thread_rng();
        new_bag.shuffle(&mut rng);
        bag.pieces = new_bag;
    }

    if let Some(piece_type) = bag.pieces.pop() {
        commands.spawn(CurrentPiece {
            piece_type,
            x: BOARD_WIDTH as i32 / 2 - 2,
            y: BOARD_HEIGHT as i32 - 4,
            rotation: 0,
        });
    }
}
