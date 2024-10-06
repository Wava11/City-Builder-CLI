use bevy::prelude::*;
use crossterm::event;

use crate::{
    geometry::{rect_contains_coords, Point},
    map::Position,
    world::{
        camera::{Camera, MoveCameraEvent},
        structure::{Pathway, Structure, StructureBundle, StructurePlacedEvent},
        Area, Rotation,
    },
};

use super::{input::KeysPressed, AppLayout, LogMessage};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_cursor)
            .add_systems(Update, (move_cursor, rotate_cursor, handle_action));
    }
}

fn init_cursor(app_layout: Res<AppLayout>, mut commands: Commands) {
    commands.spawn(CursorBundle {
        cursor: Cursor,
        position: Position(Point {
            x: app_layout.map.x as usize,
            y: app_layout.map.y as usize,
        }),
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
    app_layout: Res<AppLayout>,
    mut move_camera_events: EventWriter<MoveCameraEvent>,
    mut cursor_position_query: Query<&mut Position, With<Cursor>>,
    mut log: ResMut<LogMessage>,
) {
    let map_area = app_layout.map;
    let mut cursor_position = cursor_position_query.single_mut();

    let mut dx: isize = 0;
    let mut dy: isize = 0;

    if keys_pressed.was_char_pressed('l')
        || keys_pressed.was_key_code_pressed(event::KeyCode::Right)
    {
        dx += 1;
    }
    if keys_pressed.was_char_pressed('h') || keys_pressed.was_key_code_pressed(event::KeyCode::Left)
    {
        dx -= 1;
    }
    if keys_pressed.was_char_pressed('j') || keys_pressed.was_key_code_pressed(event::KeyCode::Down)
    {
        dy += 1;
    }
    if keys_pressed.was_char_pressed('k') || keys_pressed.was_key_code_pressed(event::KeyCode::Up) {
        dy -= 1;
    }

    let new_x = cursor_position.0.x as isize + dx;
    let new_y = cursor_position.0.y as isize + dy;

    if rect_contains_coords(map_area, new_x, new_y) {
        cursor_position.0.move_x(dx);
        cursor_position.0.move_y(dy);
    } else {
        move_camera_events.send(MoveCameraEvent { dx, dy });
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
    app_layout: Res<AppLayout>,
    mut structure_placed_events: EventWriter<StructurePlacedEvent>,
) {
    if !keys_pressed.was_char_pressed(' ') {
        return;
    }

    let (cursor_position, cursor_rotation) = cursor_query.single();
    let camera = camera_query.single();
    //why do you need to clone twice here?
    let cursor_position_in_map = Position(
        camera.top_left.clone() + cursor_position.0.clone() - app_layout.map.as_position().into(),
    );

    structure_placed_events.send(StructurePlacedEvent(StructureBundle {
        structure: Structure::Pathway(Pathway::Road),
        position: cursor_position_in_map,
        area: Area::new(1, 1),
        rotation: cursor_rotation.clone(),
    }));
}
