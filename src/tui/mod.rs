use std::io::stdout;

use bevy::prelude::*;
use crossterm::{cursor::SetCursorStyle, ExecutableCommand};
use cursor::{Cursor, CursorPlugin};
use input::{InputPlugin, KeysPressed};
use map::{create_map_view_sprite, MapView};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app::OneShotSystems, city::City, map::Position, population::Population
};

mod cursor;
mod input;
mod map;

pub struct TerminalUIPlugin;

impl Plugin for TerminalUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(( InputPlugin, CursorPlugin ))
            .add_systems(PreStartup, init_terminal)
            .add_systems(PreUpdate, create_map_view_sprite)
            .add_systems(Update, (update_terminal, exit_on_q));
    }
}

#[derive(Resource)]
pub struct Terminal(pub ratatui::DefaultTerminal);

fn init_terminal(mut commands: Commands) {
    let mut terminal = ratatui::init();
    terminal.clear().expect("could not clear terminal");
    commands.insert_resource(Terminal(terminal));
}

fn update_terminal(
    mut terminal: ResMut<Terminal>,
    population_query: Query<&Population, With<City>>,
    map_view_query: Query<&MapView>,
    cursor_query: Query<&Position, With<Cursor>>,
) {
    let Population(population) = population_query.single();
    let map_view = map_view_query.single();
    let cursor_position = cursor_query.single();
    let mut stdout = stdout();
    let _ = stdout.execute(SetCursorStyle::SteadyBlock);
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

            let map_area = layout[1];
            let absolute_cursor_position =
                cursor_position.clone() + Position::new(map_area.x, map_area.y);
            frame.render_widget_ref(map_view, map_area);
            frame.set_cursor_position(absolute_cursor_position);
        })
        .unwrap();
}

fn exit_on_q(
    one_shot_systems: Res<OneShotSystems>,
    mut commands: Commands,
    keys_pressed: Res<KeysPressed>
) {
    if keys_pressed.was_char_pressed('q') {
        commands.run_system(one_shot_systems.exit_game);
    }
}
