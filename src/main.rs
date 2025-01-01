use std::time::Duration;

use bevy::{color::palettes::tailwind::PINK_800, prelude::*, time::common_conditions::on_timer};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component)]
struct Enemy;

const FALLING_SPEED: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .add_systems(Startup, setup_camera)
        .add_systems(FixedUpdate, move_enemies_down)
        .add_systems(
            FixedUpdate,
            spawn_enemy.run_if(on_timer(Duration::from_secs(1))),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_enemy(mut commands: Commands, window: Single<&Window>) {
    let width = window.width();
    commands.spawn((
        Node {
            width: Val::Px(30.0),
            height: Val::Px(30.0),
            position_type: PositionType::Absolute,
            top: Val::Px(-30.0),
            // Generate a random left position from 0 to width
            left: Val::Px(rand::random::<f32>() * width),
            ..default()
        },
        BackgroundColor(PINK_800.into()),
        Enemy,
        Name::new("Enemy"),
    ));
}

fn move_enemies_down(mut enemies: Query<&mut Node, With<Enemy>>, time: Res<Time>) {
    for mut node in enemies.iter_mut() {
        match node.top {
            Val::Px(value) => node.top = Val::Px(value + FALLING_SPEED * time.delta_secs()),
            _ => panic!(),
        }
    }
}
