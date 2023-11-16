use bevy::{prelude::*, window::WindowMode};

mod island_map_generator_plugin;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::Fullscreen,
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins((
            DefaultPlugins.set(window_plugin),
            island_map_generator_plugin::IslandMapGeneratorPlugin,
        ))
        .run();
}
