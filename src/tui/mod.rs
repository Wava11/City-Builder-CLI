use std::io::stdout;

use bevy::prelude::*;
use crossterm::{cursor::SetCursorStyle, ExecutableCommand};
use cursor::{Cursor, CursorPlugin};
use input::{InputPlugin, KeysPressed};
use map::{create_map_view_sprite, MapView};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Rect,
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::OneShotSystems, city::City, map::Position, population::Population};

pub mod cursor;
mod input;
mod map;

pub struct TerminalUIPlugin;

impl Plugin for TerminalUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((InputPlugin, CursorPlugin))
            .add_systems(PreStartup, init_terminal)
            .add_systems(PreUpdate, create_map_view_sprite)
            .add_systems(Update, (update_terminal, update_app_layout, exit_on_q));
    }
}

#[derive(Resource)]
pub struct AppLayout {
    pub top_bar: Rect,
    pub map: Rect,
    pub log: Rect,
}

#[derive(Resource)]
pub struct LogMessage(String);

#[derive(Resource)]
pub struct Terminal(pub ratatui::DefaultTerminal);

fn init_terminal(mut commands: Commands) {
    let mut terminal = ratatui::init();
    terminal.clear().expect("could not clear terminal");
    commands.insert_resource(create_app_layout(&terminal));
    commands.insert_resource(Terminal(terminal));
    commands.insert_resource(LogMessage("".to_string()));
}

fn update_app_layout(terminal: Res<Terminal>, mut layout: ResMut<AppLayout>) {
    *layout = create_app_layout(&terminal.0);
}

fn create_app_layout(terminal: &ratatui::DefaultTerminal) -> AppLayout {
    let terminal_size = terminal.size().unwrap();
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(Rect::new(0, 0, terminal_size.width, terminal_size.height));
    AppLayout {
        top_bar: layout[0],
        map: layout[1],
        log: layout[2],
    }
}

fn update_terminal(
    mut terminal: ResMut<Terminal>,
    layout: Res<AppLayout>,
    population_query: Query<&Population, With<City>>,
    map_view_query: Query<&MapView>,
    cursor_query: Query<&Position, With<Cursor>>,
    log_message: Res<LogMessage>,
) {
    let Population(population) = population_query.single();
    let map_view = map_view_query.single();
    let cursor_position = cursor_query.single();
    let mut stdout = stdout();
    let _ = stdout.execute(SetCursorStyle::SteadyBlock);
    terminal
        .0
        .draw(|frame| {
            frame.render_widget(
                Paragraph::new(format!("Population: {population}"))
                    .block(Block::new().borders(Borders::ALL)),
                layout.top_bar,
            );

            frame.render_widget_ref(map_view, layout.map);

            frame.set_cursor_position(cursor_position.clone());

            let log = Paragraph::new(log_message.0.clone());
            frame.render_widget(log, layout.log);
        })
        .unwrap();
}

fn exit_on_q(
    one_shot_systems: Res<OneShotSystems>,
    mut commands: Commands,
    keys_pressed: Res<KeysPressed>,
) {
    if keys_pressed.was_char_pressed('q') {
        commands.run_system(one_shot_systems.exit_game);
    }
}
