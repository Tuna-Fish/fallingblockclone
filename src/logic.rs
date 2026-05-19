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
