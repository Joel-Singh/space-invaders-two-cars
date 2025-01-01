use bevy::{color::palettes::tailwind::PINK_800, prelude::*};
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
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
