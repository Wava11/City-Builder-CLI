use std::time::Duration;

use bevy::prelude::*;
use crossterm::event::{self};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeysPressed(vec![]))
            .add_systems(Update, change_pressed_keys);
    }
}

#[derive(Resource)]
pub struct KeysPressed(pub Vec<crossterm::event::KeyCode>);

impl KeysPressed {
    pub fn was_char_pressed(&self, char: char) -> bool {
        self.0.contains(&event::KeyCode::Char(char))
    }
}

fn change_pressed_keys(mut keys_pressed: ResMut<KeysPressed>) {
    keys_pressed.0.clear();
    if event::poll(Duration::from_secs(0)).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == event::KeyEventKind::Press {
                keys_pressed.0.push(key.code);
            }
        }
    }
}
