use crate::logic::{AppMode, GameAction, MovementTimer};
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

pub fn gui_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut kbd_evr: MessageReader<KeyboardInput>,
    mut actions: MessageWriter<GameAction>,
    mut exit: MessageWriter<bevy::app::AppExit>,
    app_mode: Res<AppMode>,
    time: Res<Time>,
    mut movement: ResMut<MovementTimer>,
) {
    for ev in kbd_evr.read() {
        if ev.state.is_pressed() {
            // In Bevy 0.14, KeyboardInput doesn't have a 'repeat' field.
            // We can detect OS repeats by checking if the key was already pressed
            // in the previous frame (pressed && !just_pressed).
            if keyboard.pressed(ev.key_code) && !keyboard.just_pressed(ev.key_code) {
                continue;
            }

            if ev.key_code == KeyCode::KeyQ {
                if *app_mode == AppMode::HighScore {
                    exit.write(bevy::app::AppExit::Success);
                } else {
                    actions.write(GameAction::ReturnToMenu);
                }
                return;
            }

            match *app_mode {
                AppMode::HighScore => {
                    if is_arrow(ev.key_code) || ev.key_code == KeyCode::Enter {
                        actions.write(GameAction::StartGame);
                    }
                }
                AppMode::Naming => {
                    if ev.key_code == KeyCode::Enter {
                        actions.write(GameAction::SubmitName);
                    } else if ev.key_code == KeyCode::Backspace {
                        actions.write(GameAction::Backspace);
                    } else if let Key::Character(c) = &ev.logical_key {
                        for ch in c.chars() {
                            if !ch.is_control() {
                                actions.write(GameAction::KeyPressed(ch));
                            }
                        }
                    }
                }
                AppMode::Paused => {
                    if is_arrow(ev.key_code) {
                        actions.write(GameAction::Resume);
                    }
                }
                AppMode::Playing => match ev.key_code {
                    KeyCode::KeyP => {
                        actions.write(GameAction::Pause);
                    }
                    KeyCode::ArrowDown => {
                        actions.write(GameAction::MoveDown);
                    }
                    KeyCode::ArrowUp | KeyCode::KeyZ => {
                        actions.write(GameAction::Rotate);
                    }
                    KeyCode::ArrowLeft => {
                        actions.write(GameAction::MoveLeft);
                        movement.key = Some(KeyCode::ArrowLeft);
                        movement
                            .timer
                            .set_duration(std::time::Duration::from_secs_f32(0.2));
                        movement.timer.reset();
                    }
                    KeyCode::ArrowRight => {
                        actions.write(GameAction::MoveRight);
                        movement.key = Some(KeyCode::ArrowRight);
                        movement
                            .timer
                            .set_duration(std::time::Duration::from_secs_f32(0.2));
                        movement.timer.reset();
                    }
                    _ => {}
                },
            }
        } else {
            // Released
            if *app_mode == AppMode::Playing {
                if movement.key == Some(ev.key_code) {
                    movement.key = None;
                }
            }
        }
    }

    // Continuous movement (DAS)
    if *app_mode == AppMode::Playing {
        if let Some(key) = movement.key {
            // Note: keyboard.pressed() reflects the state as of the start of this frame.
            if keyboard.pressed(key) {
                movement.timer.tick(time.delta());
                if movement.timer.just_finished() {
                    let action = if key == KeyCode::ArrowLeft {
                        GameAction::MoveLeft
                    } else {
                        GameAction::MoveRight
                    };
                    actions.write(action);
                    movement
                        .timer
                        .set_duration(std::time::Duration::from_secs_f32(0.05));
                    movement.timer.reset();
                }
            } else {
                movement.key = None;
            }
        }
    }
}

fn is_arrow(key: KeyCode) -> bool {
    matches!(
        key,
        KeyCode::ArrowUp | KeyCode::ArrowDown | KeyCode::ArrowLeft | KeyCode::ArrowRight
    )
}
