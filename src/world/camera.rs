use bevy::prelude::*;

use crate::geometry::Point;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveCameraEvent>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, move_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera {
        top_left: Point { x: 0, y: 0 },
    });
}

//for now only set the top left and what the camera can view will be detemined by the size of the
//screen
#[derive(Component)]
pub struct Camera {
    pub top_left: Point,
}

#[derive(Event)]
pub struct MoveCameraEvent {
    pub dx: isize,
    pub dy: isize,
}

fn move_camera(
    mut move_camera_events: EventReader<MoveCameraEvent>,
    mut camera_query: Query<&mut Camera>,
) {
    let mut camera_position = camera_query.single_mut();
    for ev in move_camera_events.read() {
        camera_position.top_left.move_x(ev.dx);
        camera_position.top_left.move_y(ev.dy);
    }
}
