use std::time::Duration;

use bevy::prelude::*;
use crossterm::event;
use ratatui::{style::Stylize, widgets};

use crate::{city::City, population::Population};

mod map;

pub struct TerminalUIPlugin;

impl Plugin for TerminalUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_terminal)
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

fn update_terminal(mut terminal: ResMut<Terminal>, query: Query<&Population, With<City>>) {
    let Population(population) = query.single();
    terminal
        .0
        .draw(|frame| {
            let top_bar = Block::default().title("Top bar").borders(Borders::ALL);
            f.render_widget(top_bar, size);

            // Render the "Hello" text in the app bar
            let paragraph = Paragraph::new("Hello").block(top_bar);
            f.render_widget(paragraph, size);

            let x = frame.size();
            let greeting = widgets::Paragraph::new(format!("Population: {population}!")).white();
            // .on_blue();
            frame.render_widget(greeting, frame.area());
        })
        .unwrap();
}

fn exit_on_q(
    // keys: Res<ButtonInput<KeyCode>>,
    // mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut writer: EventWriter<bevy::app::AppExit>,
) {
    if event::poll(Duration::from_secs(0)).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == event::KeyEventKind::Press && key.code == event::KeyCode::Char('q') {
                writer.send(bevy::app::AppExit);
            }
        }
    }
}