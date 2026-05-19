mod logic;

use bevy::prelude::*;
use logic::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .init_resource::<Board>()
        .init_resource::<PieceBag>()
        .add_systems(Update, spawn_piece)
        .run();
}
