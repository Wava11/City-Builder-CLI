use bevy::prelude::*;
use city::{CityPlugin};
use housing::HousingPlugin;

mod macros;
mod city;
mod housing;
mod population;
mod statistics;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CityPlugin, HousingPlugin))
        .run();
}