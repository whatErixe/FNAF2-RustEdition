use bevy::prelude::*;

mod game;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "FNAF 2: Rust Edition".into(),
                resolution: (960.0, 540.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<game::GameState>()
        .add_systems(Startup, game::setup)
        .add_systems(
            Update,
            (
                game::clock_tick,
                game::keyboard_controls,
                game::touch_controls,
                game::button_interactions,
                game::refresh_hud,
                game::animatronic_ai,
            ),
        )
        .run();
}
