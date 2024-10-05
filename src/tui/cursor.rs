use bevy::prelude::*;

use crate::{geometry::Point, map::Position};

use super::input::KeysPressed;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_cursor)
            .add_systems(Update, move_cursor);
    }
}

fn init_cursor(mut commands: Commands) {
    commands.spawn(CursorBundle {
        cursor: Cursor,
        position: Position(Point { x: 0, y: 0 }),
    });
}

#[derive(Component)]
pub struct Cursor;

#[derive(Bundle)]
struct CursorBundle {
    cursor: Cursor,
    position: Position,
}

fn move_cursor(
    keys_pressed: Res<KeysPressed>,
    mut cursor_position_query: Query<&mut Position, With<Cursor>>,
) {
    let mut cursor_position = cursor_position_query.single_mut();
    if keys_pressed.was_char_pressed('l') {
        cursor_position.0.move_x(1);
    }
    if keys_pressed.was_char_pressed('h') {
        cursor_position.0.move_x(-1);
    }
    if keys_pressed.was_char_pressed('j') {
        cursor_position.0.move_y(1);
    }
    if keys_pressed.was_char_pressed('k') {
        cursor_position.0.move_y(-1);
    }
}
