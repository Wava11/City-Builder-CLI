use std::time::Duration;

use bevy::prelude::*;
use crossterm::event;
use map::{create_map_view_sprite, MapView};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};

use crate::{city::City, population::Population};

mod map;

pub struct TerminalUIPlugin;

impl Plugin for TerminalUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_terminal)
            .add_systems(PreUpdate, create_map_view_sprite)
            .add_systems(Update, (update_terminal, exit_on_q));
    }
}

#[derive(Resource)]
struct Terminal(ratatui::DefaultTerminal);

fn init_terminal(mut commands: Commands) {
    let mut terminal = ratatui::init();
    terminal.clear().expect("could not clear terminal");
    commands.insert_resource(Terminal(terminal));
}

fn update_terminal(
    mut terminal: ResMut<Terminal>,
    population_query: Query<&Population, With<City>>,
    map_view_query: Query<&MapView>,
) {
    let Population(population) = population_query.single();
    let map_view = map_view_query.single();
    terminal
        .0
        .draw(|frame| {
            let layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
                .split(frame.area());

            frame.render_widget(
                Paragraph::new(format!("Population: {population}"))
                    .block(Block::new().borders(Borders::ALL)),
                layout[0],
            );

            frame.render_widget_ref(map_view, layout[1]);
        })
        .unwrap();
}

fn exit_on_q(mut writer: EventWriter<bevy::app::AppExit>) {
    if event::poll(Duration::from_secs(0)).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == event::KeyEventKind::Press && key.code == event::KeyCode::Char('q') {
                writer.send(bevy::app::AppExit);
            }
        }
    }
}
