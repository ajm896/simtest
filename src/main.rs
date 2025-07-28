use bevy::color::palettes::basic;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
struct Position(Vec3);
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);
#[derive(Component)]
struct Ball;
#[derive(Component)]
struct Paddle;
#[derive(Component)]
struct Player;

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
            title: "PONG".into(),
            ..default()
        }),
        ..default()
    }));

    app.add_systems(Startup, setup);
    app.add_systems(Update, move_ball);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let ball = Mesh2d(meshes.add(Rectangle::new(10., 10.)));
    let ball_mat = MeshMaterial2d(materials.add(Color::from(basic::WHITE)));
    let transform = Transform::from_xyz(0., 0., 0.);
    let init_vel = Velocity(vec3(0.5, 1., 0.));

    commands.spawn((ball, ball_mat, transform, init_vel, Ball));
}

fn move_ball(mut gizmos: Gizmos, mut ball: Query<(&mut Velocity, &mut Transform), With<Ball>>) {
    for (mut velocity, mut transform) in &mut ball {
        transform.translation += velocity.0;
        gizmos.ray(transform.translation, velocity.0 * 10., basic::LIME);
        if transform.translation.x >= WINDOW_WIDTH / 2. - 5. {
            velocity.0 = Vec3::ZERO;
            transform.translation.x = WINDOW_WIDTH / 2. - 5.; // Clamp at boundary
        }
        else if transform.translation.y >= WINDOW_HEIGHT / 2. - 5. {
            velocity.0 = Vec3::ZERO;
            transform.translation.y = WINDOW_HEIGHT / 2. - 5.; // Clamp at boundary
        }
    }
}
