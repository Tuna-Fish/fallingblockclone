use crate::logic::{
    AppMode, Board, CurrentName, CurrentPiece, GameState, HighScores, PieceBag, PieceType,
    BOARD_HEIGHT, BOARD_WIDTH,
};
use bevy::prelude::*;

pub const HUD_WIDTH: f32 = 200.0;
pub const HUD_PADDING: f32 = 40.0;

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

#[derive(Component)]
pub struct BoardBackground;

#[derive(Component)]
pub struct NextBackground;

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
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
        GameAreaBackground,
        BoardBackground,
    ));

    // Next piece background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
        GameAreaBackground,
        NextBackground,
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
    mut board_bg: Query<
        (&mut Visibility, &mut Sprite, &mut Transform),
        (With<BoardBackground>, Without<NextBackground>),
    >,
    mut next_bg: Query<
        (&mut Visibility, &mut Sprite, &mut Transform),
        (With<NextBackground>, Without<BoardBackground>),
    >,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
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

    let window = windows.single();
    let win_w = window.width();
    let win_h = window.height();

    let available_w = win_w - HUD_WIDTH - 2.0 * HUD_PADDING;
    let available_h = win_h - 2.0 * HUD_PADDING;

    // Calculate dynamic block size based on 10x20 aspect ratio
    let block_size = (available_w / 10.0).min(available_h / 20.0).max(10.0);

    // Board offsets (relative to center of window)
    let board_center_x = -win_w / 2.0 + HUD_PADDING + (available_w / 2.0);
    let board_center_y = 0.0;

    let board_bottom_left_x = board_center_x - (5.0 * block_size);
    let board_bottom_left_y = board_center_y - (10.0 * block_size);

    let is_visible = *app_mode == AppMode::Playing || *app_mode == AppMode::Paused;

    // Update board background
    if let Ok((mut vis, mut sprite, mut trans)) = board_bg.get_single_mut() {
        *vis = if is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        sprite.custom_size = Some(Vec2::new(block_size * 10.0, block_size * 20.0));
        trans.translation = Vec3::new(board_center_x, board_center_y, -1.0);
    }

    // Update next piece background
    let next_bg_center_x = board_center_x + (5.0 * block_size) + HUD_PADDING + (HUD_WIDTH / 2.0);
    let next_bg_center_y = win_h / 2.0 - HUD_PADDING - (2.0 * block_size);

    if let Ok((mut vis, mut sprite, mut trans)) = next_bg.get_single_mut() {
        *vis = if is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        sprite.custom_size = Some(Vec2::new(block_size * 4.0, block_size * 4.0));
        trans.translation = Vec3::new(next_bg_center_x, next_bg_center_y, -1.0);
    }

    if !is_visible {
        return;
    }

    // Draw board blocks
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            if let Some(piece_type) = board.grid[y][x] {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: get_color(piece_type),
                            custom_size: Some(Vec2::new(block_size - 2.0, block_size - 2.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            board_bottom_left_x + (x as f32 + 0.5) * block_size,
                            board_bottom_left_y + (y as f32 + 0.5) * block_size,
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
                            custom_size: Some(Vec2::new(block_size - 2.0, block_size - 2.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            board_bottom_left_x + (nx as f32 + 0.5) * block_size,
                            board_bottom_left_y + (ny as f32 + 0.5) * block_size,
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
        let next_start_x = next_bg_center_x - (2.0 * block_size);
        let next_start_y = next_bg_center_y - (2.0 * block_size);

        for (dx, dy) in next_piece.coordinates(0) {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: get_color(*next_piece),
                        custom_size: Some(Vec2::new(block_size - 2.0, block_size - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        next_start_x + (dx as f32 + 0.5) * block_size,
                        next_start_y + (dy as f32 + 0.5) * block_size,
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
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::srgb(1.0, 1.0, 1.0),
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(30.0),
                        left: Val::Percent(20.0),
                        padding: UiRect::all(Val::Px(40.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                HighScoreText,
            ));
        });
}

pub fn update_ui(
    mut score_text: Query<&mut Text, (With<ScoreText>, Without<HighScoreText>)>,
    mut high_score_text: Query<(&mut Text, &mut Visibility), With<HighScoreText>>,
    mut hud_container: Query<&mut Style, With<HudContainer>>,
    game_state: Res<GameState>,
    app_mode: Res<AppMode>,
    high_scores: Res<HighScores>,
    current_name: Res<CurrentName>,
    timer: Res<crate::logic::GravityTimer>,
    mut clear_color: ResMut<ClearColor>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    // Update clear color based on speed scaling
    clear_color.0 = PASTEL_COLORS[game_state.color_index];

    let window = windows.single();
    let win_w = window.width();
    let win_h = window.height();
    let available_w = win_w - HUD_WIDTH - 2.0 * HUD_PADDING;
    let available_h = win_h - 2.0 * HUD_PADDING;
    let block_size = (available_w / 10.0).min(available_h / 20.0).max(10.0);

    if let Ok(mut style) = hud_container.get_single_mut() {
        if *app_mode == AppMode::Naming {
            style.right = Val::Auto;
            style.left = Val::Percent(35.0);
            style.top = Val::Percent(40.0);
        } else {
            style.right = Val::Auto;
            style.left = Val::Px(HUD_PADDING * 2.0 + (available_w / 2.0) + 5.0 * block_size);
            style.top = Val::Px(HUD_PADDING + 4.0 * block_size + 20.0);
        }
    }

    if let Ok(mut text) = score_text.get_single_mut() {
        match *app_mode {
            AppMode::Playing | AppMode::Paused => {
                text.sections[0].value = format!(
                    "Score: {:>6}\nLines: {:>6}\nDelay: {:>5.2}s\n{}",
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
                    "CONGRATULATIONS!\nScore: {:>5}\nEnter Name: {}",
                    game_state.score, current_name.0
                );
            }
            AppMode::HighScore => {
                text.sections[0].value = format!("Last Score: {:>5}", game_state.score);
            }
        }
    }

    if let Ok((mut text, mut visibility)) = high_score_text.get_single_mut() {
        if *app_mode == AppMode::HighScore {
            let mut content = String::from("--- HIGH SCORES ---\n");
            for (i, (name, score)) in high_scores.0.iter().enumerate() {
                content.push_str(&format!("{:2}. {:<10} {:>5}\n", i + 1, name, score));
            }
            content.push_str("\nPress any key to Start");
            text.sections[0].value = content;
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
