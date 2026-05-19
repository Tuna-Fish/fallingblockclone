mod input;
mod logic;
mod rendering;

use bevy::prelude::*;
use input::*;
use logic::*;
use rendering::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Tetris".into(),
                resolution: (600.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_event::<GameAction>()
        .init_resource::<Board>()
        .init_resource::<PieceBag>()
        .init_resource::<GravityTimer>()
        .init_resource::<GameState>()
        .init_resource::<AppMode>()
        .init_resource::<HighScores>()
        .init_resource::<CurrentName>()
        .add_systems(Startup, (setup_camera, setup_backgrounds, setup_ui))
        .add_systems(
            Update,
            (
                gui_input,
                apply_gravity,
                handle_actions,
                spawn_piece,
                render_board,
                update_ui,
            )
                .chain(),
        )
        .run();
}
