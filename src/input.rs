use crate::logic::{AppMode, GameAction, MovementTimer};
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

pub fn gui_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut kbd_evr: EventReader<KeyboardInput>,
    mut actions: EventWriter<GameAction>,
    mut exit: EventWriter<bevy::app::AppExit>,
    app_mode: Res<AppMode>,
    time: Res<Time>,
    mut movement: ResMut<MovementTimer>,
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        exit.send(bevy::app::AppExit::Success);
        return;
    }

    match *app_mode {
        AppMode::HighScore => {
            if keyboard.get_just_pressed().next().is_some() {
                actions.send(GameAction::StartGame);
            }
        }
        AppMode::Naming => {
            if keyboard.just_pressed(KeyCode::Enter) {
                actions.send(GameAction::SubmitName);
            } else if keyboard.just_pressed(KeyCode::Backspace) {
                actions.send(GameAction::Backspace);
            } else {
                for ev in kbd_evr.read() {
                    if ev.state.is_pressed() {
                        if let Key::Character(c) = &ev.logical_key {
                            for ch in c.chars() {
                                if !ch.is_control() {
                                    actions.send(GameAction::KeyPressed(ch));
                                }
                            }
                        }
                    }
                }
            }
        }
        AppMode::Paused => {
            if keyboard.just_pressed(KeyCode::ArrowUp)
                || keyboard.just_pressed(KeyCode::ArrowDown)
                || keyboard.just_pressed(KeyCode::ArrowLeft)
                || keyboard.just_pressed(KeyCode::ArrowRight)
            {
                actions.send(GameAction::Resume);
            }
        }
        AppMode::Playing => {
            if keyboard.just_pressed(KeyCode::KeyP) {
                actions.send(GameAction::Pause);
            } else if keyboard.just_pressed(KeyCode::ArrowDown) {
                actions.send(GameAction::MoveDown);
            } else if keyboard.just_pressed(KeyCode::ArrowUp)
                || keyboard.just_pressed(KeyCode::KeyZ)
            {
                actions.send(GameAction::Rotate);
            }

            // Continuous horizontal movement (DAS)
            let left = keyboard.pressed(KeyCode::ArrowLeft);
            let right = keyboard.pressed(KeyCode::ArrowRight);

            if left && right {
                movement.key = None;
            } else if left {
                handle_movement(
                    KeyCode::ArrowLeft,
                    GameAction::MoveLeft,
                    &keyboard,
                    &time,
                    &mut movement,
                    &mut actions,
                );
            } else if right {
                handle_movement(
                    KeyCode::ArrowRight,
                    GameAction::MoveRight,
                    &keyboard,
                    &time,
                    &mut movement,
                    &mut actions,
                );
            } else {
                movement.key = None;
            }
        }
    }
}

fn handle_movement(
    key: KeyCode,
    action: GameAction,
    keyboard: &Res<ButtonInput<KeyCode>>,
    time: &Res<Time>,
    movement: &mut ResMut<MovementTimer>,
    actions: &mut EventWriter<GameAction>,
) {
    if keyboard.just_pressed(key) {
        actions.send(action);
        movement.key = Some(key);
        movement
            .timer
            .set_duration(std::time::Duration::from_secs_f32(0.2));
        movement.timer.reset();
    } else if movement.key == Some(key) {
        movement.timer.tick(time.delta());
        if movement.timer.finished() {
            actions.send(action);
            movement
                .timer
                .set_duration(std::time::Duration::from_secs_f32(0.05));
            movement.timer.reset();
        }
    }
}
