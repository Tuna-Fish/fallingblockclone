use crate::logic::GameAction;
use bevy::prelude::*;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn terminal_input(mut actions: EventWriter<GameAction>) {
    if event::poll(Duration::from_millis(0)).unwrap_or(false) {
        if let Ok(Event::Key(key)) = event::read() {
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
