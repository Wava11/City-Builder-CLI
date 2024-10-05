use app::AppPlugin;
use bevy::{log::LogPlugin, prelude::*};
use city::CityPlugin;
use housing::HousingPlugin;
use tui::TerminalUIPlugin;
use world::WorldPlugin;

mod city;
mod housing;
mod map;
mod macros;
mod world;
mod population;
mod statistics;
mod tui;
mod geometry;
mod app;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: None,
                exit_condition: bevy::window::ExitCondition::DontExit,
                close_when_requested: false,
            }).disable::<LogPlugin>(),
            AppPlugin,
            CityPlugin,
            HousingPlugin,
            TerminalUIPlugin,
            WorldPlugin,
        ))
        .run();
}
