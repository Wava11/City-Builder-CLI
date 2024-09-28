use bevy::prelude::*;
use city::CityPlugin;
use housing::HousingPlugin;
use tui::TerminalUIPlugin;

mod city;
mod housing;
mod macros;
mod population;
mod statistics;
mod tui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: None,
                exit_condition: bevy::window::ExitCondition::DontExit,
                close_when_requested: false,
            }),
            CityPlugin,
            HousingPlugin,
            TerminalUIPlugin,
        ))
        .run();
}
