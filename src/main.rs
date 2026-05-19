mod input;
mod logic;
mod rendering;

use bevy::prelude::*;
use input::*;
use logic::*;
use rendering::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_event::<GameAction>()
        .init_resource::<Board>()
        .init_resource::<PieceBag>()
        .init_resource::<GravityTimer>()
        .add_systems(Startup, setup_terminal)
        .add_systems(
            Update,
            (
                terminal_input,
                apply_gravity,
                handle_actions,
                spawn_piece,
                render_system,
                restore_terminal,
            )
                .chain(),
        )
        .run();
}
