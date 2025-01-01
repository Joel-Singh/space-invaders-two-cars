use bevy::{
    color::palettes::tailwind::{GRAY_400, PINK_800},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod spawning_enemies;
use spawning_enemies::spawning_enemies;

const FALLING_SPEED: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            spawning_enemies,
        ))
        .add_systems(Startup, (setup_camera, spawn_cannon))
        .run();
}

fn spawn_cannon(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Vw(100.0),
                margin: UiRect::top(Val::Auto),
                justify_content: JustifyContent::Center,
                ..default()
            },
            Name::new("Cannon Container"),
        ))
        .with_child((
            Node {
                width: Val::Px(50.0),
                height: Val::Px(100.0),
                ..default()
            },
            BackgroundColor(GRAY_400.into()),
            Name::new("Cannon"),
        ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
