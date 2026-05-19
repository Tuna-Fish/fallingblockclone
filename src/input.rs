use crate::logic::{AppMode, GameAction};
use bevy::prelude::*;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn terminal_input(
    mut actions: EventWriter<GameAction>,
    mut exit: EventWriter<bevy::app::AppExit>,
    app_mode: Res<AppMode>,
) {
    if event::poll(Duration::from_millis(0)).unwrap_or(false) {
        if let Ok(Event::Key(key)) = event::read() {
            if key.code == KeyCode::Char('q') {
                exit.send(bevy::app::AppExit::Success);
                return;
            }

            match *app_mode {
                AppMode::HighScore => match key.code {
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Enter => {
                        actions.send(GameAction::StartGame);
                    }
                    _ => {}
                },
                AppMode::Naming => match key.code {
                    KeyCode::Enter => {
                        actions.send(GameAction::SubmitName);
                    }
                    KeyCode::Backspace => {
                        actions.send(GameAction::Backspace);
                    }
                    KeyCode::Char(c) => {
                        actions.send(GameAction::KeyPressed(c));
                    }
                    _ => {}
                },
                AppMode::Paused => match key.code {
                    KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                        actions.send(GameAction::Resume);
                    }
                    _ => {}
                },
                AppMode::Playing => {
                    if key.code == KeyCode::Char('p') {
                        actions.send(GameAction::Pause);
                        return;
                    }
                    let action = match key.code {
                        KeyCode::Left => Some(GameAction::MoveLeft),
                        KeyCode::Right => Some(GameAction::MoveRight),
                        KeyCode::Down => Some(GameAction::MoveDown),
                        KeyCode::Up | KeyCode::Char('z') => Some(GameAction::Rotate),
                        _ => None,
                    };
                    if let Some(action) = action {
                        actions.send(action);
                    }
                }
            }
        }
    }
}
