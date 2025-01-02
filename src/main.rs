use std::f32::consts::FRAC_PI_2;

use bevy::{
    color::palettes::tailwind::{GRAY_400, GRAY_900, PINK_800},
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod spawning_enemies;
use spawning_enemies::spawning_enemies;

const FALLING_SPEED: f32 = 100.0;

#[derive(Component)]
struct Cannon;

#[derive(Component)]
struct Cannonball;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            spawning_enemies,
        ))
        .add_systems(Startup, (setup_camera, spawn_cannon))
        .add_systems(
            FixedUpdate,
            (
                spawn_cannonball.run_if(input_just_pressed(KeyCode::Space)),
                move_cannonballs_forward,
            ),
        )
        .run();
}

const CANNON_HEIGHT: f32 = 100.0;
const CANNON_WIDTH: f32 = 50.0;

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
                width: Val::Px(CANNON_WIDTH),
                height: Val::Px(CANNON_HEIGHT),
                ..default()
            },
            BackgroundColor(GRAY_400.into()),
            Name::new("Cannon"),
            Cannon,
        ));
}

fn spawn_cannonball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let window_height = window.height();

    let color: Color = GRAY_900.into();
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(color)),
        Cannonball,
        Transform {
            translation: Vec3::new(0.0, CANNON_HEIGHT + 30.0 - window_height / 2.0, 0.0),
            rotation: Quat::from_rotation_z(0.0),
            ..default()
        },
        Name::new("Cannonball"),
    ));
}

fn move_cannonballs_forward(
    mut cannonballs: Query<&mut Transform, With<Cannonball>>,
    time: Res<Time>,
) {
    for mut cannonball in cannonballs.iter_mut() {
        let forward = cannonball.local_x();
        const SPEED: f32 = 100.0;
        cannonball.translation += forward * SPEED * time.delta_secs();
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
