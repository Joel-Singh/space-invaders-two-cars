use bevy::{
    color::palettes::tailwind::{GRAY_400, GRAY_900},
    input::common_conditions::input_just_pressed,
    prelude::*,
};

#[derive(Component)]
struct Cannon;

#[derive(Component)]
struct Cannonball;

const CANNON_HEIGHT: f32 = 100.0;
const CANNON_WIDTH: f32 = 50.0;

pub fn cannon(app: &mut App) {
    app.add_systems(Startup, spawn_cannon).add_systems(
        FixedUpdate,
        (
            spawn_cannonball.run_if(input_just_pressed(KeyCode::Space)),
            move_cannonballs_forward,
        ),
    );
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
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
) {
    let (camera, camera_transform) = *camera_query;

    let cursor_position_as_world = camera
        .viewport_to_world_2d(
            camera_transform,
            window.cursor_position().unwrap_or(Vec2::ZERO),
        )
        .unwrap();

    let translation = Vec3::new(0.0, CANNON_HEIGHT + 30.0 - window.height() / 2.0, 0.0);

    let angle_to_face_cursor = {
        let x = cursor_position_as_world.x - translation.x;
        let y = cursor_position_as_world.y - translation.y;
        Quat::from_rotation_z(y.atan2(x))
    };

    let color: Color = GRAY_900.into();
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(color)),
        Cannonball,
        Transform {
            translation,
            rotation: angle_to_face_cursor,
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
