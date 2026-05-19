use crate::logic::{
    AppMode, Board, CurrentName, CurrentPiece, GameState, HighScores, PieceBag, PieceType,
    BOARD_HEIGHT, BOARD_WIDTH,
};
use bevy::prelude::*;

pub const BLOCK_SIZE: f32 = 30.0;
pub const BOARD_OFFSET_X: f32 = -150.0;
pub const BOARD_OFFSET_Y: f32 = -300.0;

#[derive(Component)]
pub struct BoardBlock;

#[derive(Component)]
pub struct CurrentPieceBlock;

#[derive(Component)]
pub struct NextPieceBlock;

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HighScoreText;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn get_color(piece_type: PieceType) -> Color {
    match piece_type {
        PieceType::I => Color::srgb(0.0, 1.0, 1.0),
        PieceType::O => Color::srgb(1.0, 1.0, 0.0),
        PieceType::T => Color::srgb(0.5, 0.0, 0.5),
        PieceType::S => Color::srgb(0.0, 1.0, 0.0),
        PieceType::Z => Color::srgb(1.0, 0.0, 0.0),
        PieceType::J => Color::srgb(0.0, 0.0, 1.0),
        PieceType::L => Color::srgb(1.0, 0.5, 0.0),
    }
}

pub fn render_board(
    mut commands: Commands,
    board: Res<Board>,
    current_piece: Query<&CurrentPiece>,
    bag: Res<PieceBag>,
    app_mode: Res<AppMode>,
    board_blocks: Query<Entity, With<BoardBlock>>,
    current_blocks: Query<Entity, With<CurrentPieceBlock>>,
    next_blocks: Query<Entity, With<NextPieceBlock>>,
) {
    // Cleanup previous frame blocks
    for entity in board_blocks.iter() {
        commands.entity(entity).despawn();
    }
    for entity in current_blocks.iter() {
        commands.entity(entity).despawn();
    }
    for entity in next_blocks.iter() {
        commands.entity(entity).despawn();
    }

    if *app_mode != AppMode::Playing && *app_mode != AppMode::Paused {
        return;
    }

    // Draw board
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            if let Some(piece_type) = board.grid[y][x] {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: get_color(piece_type),
                            custom_size: Some(Vec2::new(BLOCK_SIZE - 2.0, BLOCK_SIZE - 2.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            BOARD_OFFSET_X + x as f32 * BLOCK_SIZE,
                            BOARD_OFFSET_Y + y as f32 * BLOCK_SIZE,
                            0.0,
                        ),
                        ..default()
                    },
                    BoardBlock,
                ));
            }
        }
    }

    // Draw current piece
    if let Ok(piece) = current_piece.get_single() {
        for (dx, dy) in piece.piece_type.coordinates(piece.rotation) {
            let nx = piece.x + dx;
            let ny = piece.y + dy;
            if nx >= 0 && nx < BOARD_WIDTH as i32 && ny >= 0 && ny < BOARD_HEIGHT as i32 {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: get_color(piece.piece_type),
                            custom_size: Some(Vec2::new(BLOCK_SIZE - 2.0, BLOCK_SIZE - 2.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            BOARD_OFFSET_X + nx as f32 * BLOCK_SIZE,
                            BOARD_OFFSET_Y + ny as f32 * BLOCK_SIZE,
                            1.0,
                        ),
                        ..default()
                    },
                    CurrentPieceBlock,
                ));
            }
        }
    }

    // Draw next piece
    if let Some(next_piece) = bag.pieces.last() {
        for (dx, dy) in next_piece.coordinates(0) {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: get_color(*next_piece),
                        custom_size: Some(Vec2::new(BLOCK_SIZE - 2.0, BLOCK_SIZE - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        BOARD_OFFSET_X + (BOARD_WIDTH as f32 + 2.0 + dx as f32) * BLOCK_SIZE,
                        BOARD_OFFSET_Y + (BOARD_HEIGHT as f32 - 5.0 + dy as f32) * BLOCK_SIZE,
                        0.0,
                    ),
                    ..default()
                },
                NextPieceBlock,
            ));
        }
    }
}

pub fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            UiRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ScoreText,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::srgb(1.0, 1.0, 0.0),
                        ..default()
                    },
                ),
                HighScoreText,
            ));
        });
}

pub fn update_ui(
    mut score_text: Query<&mut Text, (With<ScoreText>, Without<HighScoreText>)>,
    mut high_score_text: Query<&mut Text, With<HighScoreText>>,
    game_state: Res<GameState>,
    app_mode: Res<AppMode>,
    high_scores: Res<HighScores>,
    current_name: Res<CurrentName>,
) {
    if let Ok(mut text) = score_text.get_single_mut() {
        match *app_mode {
            AppMode::Playing | AppMode::Paused => {
                text.sections[0].value = format!(
                    "Score: {}\nLines: {}\n{}",
                    game_state.score,
                    game_state.lines,
                    if *app_mode == AppMode::Paused {
                        "PAUSED"
                    } else {
                        ""
                    }
                );
            }
            AppMode::Naming => {
                text.sections[0].value = format!(
                    "CONGRATULATIONS!\nScore: {}\nEnter Name: {}",
                    game_state.score, current_name.0
                );
            }
            AppMode::HighScore => {
                text.sections[0].value = format!("Last Score: {}", game_state.score);
            }
        }
    }

    if let Ok(mut text) = high_score_text.get_single_mut() {
        if *app_mode == AppMode::HighScore {
            let mut content = String::from("--- HIGH SCORES ---\n");
            for (i, (name, score)) in high_scores.0.iter().enumerate() {
                content.push_str(&format!("{}. {:<10} {}\n", i + 1, name, score));
            }
            content.push_str("\nPress any key to Start");
            text.sections[0].value = content;
        } else {
            text.sections[0].value = "".to_string();
        }
    }
}
