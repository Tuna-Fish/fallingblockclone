use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup() {
    println!("Bevy app initialized!");
}
