use bevy::{ecs::system::SystemId, prelude::*};

use crate::tui::Terminal;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OneShotSystems>();
    }
}

#[derive(Resource)]
pub struct OneShotSystems {
    pub exit_game: SystemId,
}

impl FromWorld for OneShotSystems {
    fn from_world(world: &mut World) -> Self {
        OneShotSystems {
            exit_game: world.register_system(exit_game),
        }
    }
}

fn exit_game(mut writer: EventWriter<bevy::app::AppExit>, mut terminal: ResMut<Terminal>) {
    let _ = terminal.0.clear();
    writer.send(bevy::app::AppExit);
}
