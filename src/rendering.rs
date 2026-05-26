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

#[derive(Component)]
pub struct GameAreaBackground;

#[derive(Component)]
pub struct HudContainer;

pub const PASTEL_COLORS: [Color; 20] = [
    Color::srgb(0.7, 0.9, 0.7),
    Color::srgb(0.7, 0.7, 0.9),
    Color::srgb(0.9, 0.9, 0.7),
    Color::srgb(0.8, 0.7, 0.9),
    Color::srgb(0.7, 0.9, 0.9),
    Color::srgb(0.9, 0.8, 0.7),
    Color::srgb(0.7, 0.8, 0.8),
    Color::srgb(0.8, 0.8, 0.9),
    Color::srgb(0.8, 0.9, 0.8),
    Color::srgb(1.0, 0.8, 0.8),
    Color::srgb(1.0, 0.7, 0.8),
    Color::srgb(0.8, 0.8, 1.0),
    Color::srgb(1.0, 1.0, 0.8),
    Color::srgb(0.8, 1.0, 1.0),
    Color::srgb(1.0, 0.8, 0.7),
    Color::srgb(0.8, 0.8, 0.7),
    Color::srgb(0.8, 0.9, 1.0),
    Color::srgb(0.9, 0.8, 1.0),
    Color::srgb(0.7, 0.7, 0.7),
    Color::srgb(0.9, 0.7, 0.7),
];

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_backgrounds(mut commands: Commands) {
    // Board background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(
                    BLOCK_SIZE * BOARD_WIDTH as f32,
                    BLOCK_SIZE * BOARD_HEIGHT as f32,
                )),
                ..default()
            },
            transform: Transform::from_xyz(-15.0, -15.0, -1.0),
            ..default()
        },
        GameAreaBackground,
    ));

    // Next piece background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(BLOCK_SIZE * 4.0, BLOCK_SIZE * 4.0)),
                ..default()
            },
            transform: Transform::from_xyz(255.0, 195.0, -1.0),
            ..default()
        },
        GameAreaBackground,
    ));
}

pub fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<bevy::winit::WinitWindows>,
) {
    for window in windows.windows.values() {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::load_from_memory(include_bytes!("../assets/icon.png"))
                .unwrap()
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        let icon = winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
        window.set_window_icon(Some(icon));
    }
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
    mut backgrounds: Query<&mut Visibility, With<GameAreaBackground>>,
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

    let is_visible = *app_mode == AppMode::Playing || *app_mode == AppMode::Paused;
    for mut visibility in backgrounds.iter_mut() {
        *visibility = if is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if !is_visible {
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
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            UiRoot,
        ))
        .with_children(|parent| {
            // HUD Background
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(10.0),
                            right: Val::Px(10.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLACK),
                        ..default()
                    },
                    HudContainer,
                ))
                .with_children(|hud| {
                    hud.spawn((
                        TextBundle::from_section(
                            "INITIALIZING...",
                            TextStyle {
                                font_size: 25.0,
                                color: Color::srgb(1.0, 1.0, 1.0),
                                ..default()
                            },
                        ),
                        ScoreText,
                    ));
                });

            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::srgb(1.0, 1.0, 0.0),
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(30.0),
                    left: Val::Percent(20.0),
                    ..default()
                }),
                HighScoreText,
            ));
        });
}

pub fn update_ui(
    mut score_text: Query<&mut Text, (With<ScoreText>, Without<HighScoreText>)>,
    mut high_score_text: Query<&mut Text, With<HighScoreText>>,
    mut hud_container: Query<&mut Style, With<HudContainer>>,
    game_state: Res<GameState>,
    app_mode: Res<AppMode>,
    high_scores: Res<HighScores>,
    current_name: Res<CurrentName>,
    timer: Res<crate::logic::GravityTimer>,
    mut clear_color: ResMut<ClearColor>,
) {
    // Update clear color based on speed scaling
    clear_color.0 = PASTEL_COLORS[game_state.color_index];

    if let Ok(mut style) = hud_container.get_single_mut() {
        if *app_mode == AppMode::Naming {
            style.right = Val::Auto;
            style.left = Val::Percent(35.0);
            style.top = Val::Percent(40.0);
        } else {
            style.right = Val::Px(10.0);
            style.left = Val::Auto;
            style.top = Val::Px(10.0);
        }
    }

    if let Ok(mut text) = score_text.get_single_mut() {
        match *app_mode {
            AppMode::Playing | AppMode::Paused => {
                text.sections[0].value = format!(
                    "Score: {}\nLines: {}\nDelay: {:.2}s\n{}",
                    game_state.score,
                    game_state.lines,
                    timer.0.duration().as_secs_f32(),
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
                content.push_str(&format!("{:2}. {:<10} {:4}\n", i + 1, name, score));
            }
            content.push_str("\nPress any key to Start");
            text.sections[0].value = content;
        } else {
            text.sections[0].value = "".to_string();
        }
    }
}
