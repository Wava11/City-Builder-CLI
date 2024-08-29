use bevy::prelude::*;
use city::{CityPlugin};

mod macros;
mod city;
mod housing;
mod population;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CityPlugin))
        .run();
}
