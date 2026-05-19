use crate::logic::{AppMode, GameAction};
use bevy::prelude::*;

pub fn gui_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut actions: EventWriter<GameAction>,
    mut exit: EventWriter<bevy::app::AppExit>,
    app_mode: Res<AppMode>,
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
                for ev in char_evr.read() {
                    for c in ev.char.chars() {
                        if !c.is_control() {
                            actions.send(GameAction::KeyPressed(c));
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
            } else if keyboard.just_pressed(KeyCode::ArrowLeft) {
                actions.send(GameAction::MoveLeft);
            } else if keyboard.just_pressed(KeyCode::ArrowRight) {
                actions.send(GameAction::MoveRight);
            } else if keyboard.just_pressed(KeyCode::ArrowDown) {
                actions.send(GameAction::MoveDown);
            } else if keyboard.just_pressed(KeyCode::ArrowUp)
                || keyboard.just_pressed(KeyCode::KeyZ)
            {
                actions.send(GameAction::Rotate);
            }
        }
    }
}
