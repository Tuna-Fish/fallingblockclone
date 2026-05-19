mod input;
mod logic;

use bevy::prelude::*;
use input::*;
use logic::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_event::<GameAction>()
        .init_resource::<Board>()
        .init_resource::<PieceBag>()
        .add_systems(
            Update,
            (terminal_input, handle_actions, spawn_piece).chain(),
        )
        .run();
}
