
use bevy::{color::palettes::tailwind::PINK_800, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component)]
struct Enemy;


const FALLING_SPEED: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .add_systems(Startup, setup_camera)
        .add_systems(FixedUpdate, move_enemies_down)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Node {
            width: Val::Px(30.0),
            height: Val::Px(30.0),
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            ..default()
        },
        BackgroundColor(PINK_800.into()),
        Enemy,
        Name::new("Test Enemy")
    ));
}

fn move_enemies_down(mut enemies: Query<&mut Node, With<Enemy>>, time: Res<Time>) {
    for mut node in enemies.iter_mut() {
        match node.top {
            Val::Px(value) => {
                node.top = Val::Px(value + FALLING_SPEED * time.delta_secs())
            },
            _ => panic!(),
        }
    }
}
