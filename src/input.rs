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
    for ev in kbd_evr.read() {
        if ev.state.is_pressed() {
            // Ignore OS auto-repeat to let our own DAS handle it
            if keyboard.pressed(ev.key_code) {
                continue;
            }

            if ev.key_code == KeyCode::KeyQ {
                exit.send(bevy::app::AppExit::Success);
                return;
            }

            match *app_mode {
                AppMode::HighScore => {
                    actions.send(GameAction::StartGame);
                }
                AppMode::Naming => {
                    if ev.key_code == KeyCode::Enter {
                        actions.send(GameAction::SubmitName);
                    } else if ev.key_code == KeyCode::Backspace {
                        actions.send(GameAction::Backspace);
                    } else if let Key::Character(c) = &ev.logical_key {
                        for ch in c.chars() {
                            if !ch.is_control() {
                                actions.send(GameAction::KeyPressed(ch));
                            }
                        }
                    }
                }
                AppMode::Paused => {
                    if is_arrow(ev.key_code) {
                        actions.send(GameAction::Resume);
                    }
                }
                AppMode::Playing => match ev.key_code {
                    KeyCode::KeyP => {
                        actions.send(GameAction::Pause);
                    }
                    KeyCode::ArrowDown => {
                        actions.send(GameAction::MoveDown);
                    }
                    KeyCode::ArrowUp | KeyCode::KeyZ => {
                        actions.send(GameAction::Rotate);
                    }
                    KeyCode::ArrowLeft => {
                        actions.send(GameAction::MoveLeft);
                        movement.key = Some(KeyCode::ArrowLeft);
                        movement
                            .timer
                            .set_duration(std::time::Duration::from_secs_f32(0.2));
                        movement.timer.reset();
                    }
                    KeyCode::ArrowRight => {
                        actions.send(GameAction::MoveRight);
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
                if movement.timer.finished() {
                    let action = if key == KeyCode::ArrowLeft {
                        GameAction::MoveLeft
                    } else {
                        GameAction::MoveRight
                    };
                    actions.send(action);
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
