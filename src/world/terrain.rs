use bevy::prelude::*;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

fn hello() {
    let x = Paragraph::new("Hello");
}

pub enum TerrainType {
    Ground,
    Water,
    Building,
}

impl TerrainType {
    pub fn to_chr(&self) -> char {
        match self {
            Self::Ground => '.',
            Self::Water => '*',
            Self::Building => '#',
        }
    }
}
