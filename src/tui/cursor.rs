use bevy::prelude::*;

use crate::{
    geometry::Point,
    map::Position,
    world::{
        structure::{Pathway, Structure, StructureBundle, StructurePlacedEvent},
        Area, Camera, Rotation,
    },
};

use super::input::KeysPressed;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_cursor)
            .add_systems(Update, (move_cursor, rotate_cursor, handle_action));
    }
}

fn init_cursor(mut commands: Commands) {
    commands.spawn(CursorBundle {
        cursor: Cursor,
        position: Position(Point { x: 0, y: 0 }),
        rotation: Rotation::Right,
    });
}

#[derive(Component)]
pub struct Cursor;

#[derive(Bundle)]
struct CursorBundle {
    cursor: Cursor,
    position: Position,
    rotation: Rotation,
}

fn move_cursor(
    keys_pressed: Res<KeysPressed>,
    mut cursor_position_query: Query<&mut Position, With<Cursor>>,
) {
    //TODO: make it so cursor can't go out of bounds of map
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

fn rotate_cursor(
    keys_pressed: Res<KeysPressed>,
    mut cursor_rotation_query: Query<&mut Rotation, With<Cursor>>,
) {
    let mut cursor_rotation = cursor_rotation_query.single_mut();

    if keys_pressed.was_char_pressed('r') {
        *cursor_rotation = cursor_rotation.next_clockwise();
    }
}

fn handle_action(
    keys_pressed: Res<KeysPressed>,
    cursor_query: Query<(&Position, &Rotation), With<Cursor>>,
    camera_query: Query<&Camera>,
    mut structure_placed_events: EventWriter<StructurePlacedEvent>,
) {
    if !keys_pressed.was_char_pressed(' ') {
        return;
    }

    let (cursor_position, cursor_rotation) = cursor_query.single();
    let camera = camera_query.single();
    //why do you need to clone twice here?
    let cursor_position_in_map = Position(camera.top_left.clone() + cursor_position.0.clone());

    structure_placed_events.send(StructurePlacedEvent(StructureBundle {
        structure: Structure::Pathway(Pathway::Road),
        position: cursor_position_in_map,
        area: Area::new(1, 1),
        rotation: cursor_rotation.clone(),
    }));
}
