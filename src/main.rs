use bevy::color::palettes::basic;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
struct Position(Vec3);
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);
#[derive(Component)]
struct Ball;
#[derive(Component)]
struct Paddle;
#[derive(Component)]
struct Player;

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;
const BALL_SIZE: f32 = 10.;
const BALL_SPEED: f32 = 5.;

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

    app.add_systems(Startup, (setup, make_ball, make_paddle, make_cpu_paddle));
    app.add_systems(Update, move_ball);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn move_ball(mut gizmos: Gizmos, mut ball: Query<(&mut Velocity, &mut Transform), With<Ball>>) {
    for (mut velocity, mut transform) in &mut ball {
        transform.translation += velocity.0.extend(0.);
        gizmos.ray(
            transform.translation,
            velocity.0.extend(0.) * 10.,
            basic::LIME,
        );
        if transform.translation.x >= WINDOW_WIDTH / 2. - 5. {
            velocity.0 = velocity.0.reflect(Vec2 { x: -1., y: 0. });
        } else if transform.translation.y >= WINDOW_HEIGHT / 2. - 5. {
            velocity.0 = velocity.0.reflect(Vec2 { y: -1., x: 0. });
        } else if transform.translation.x <= -WINDOW_WIDTH / 2. + 5. {
            velocity.0 = velocity.0.reflect(Vec2 { x: -1., y: 0. });
        } else if transform.translation.y <= -WINDOW_HEIGHT / 2. + 5. {
            velocity.0 = velocity.0.reflect(Vec2 { y: -1., x: 0. });
        }
    }
}

fn make_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ball = Mesh2d(meshes.add(Rectangle::new(BALL_SIZE, BALL_SIZE)));
    let ball_mat = MeshMaterial2d(materials.add(Color::from(basic::WHITE)));
    let transform = Transform::from_xyz(0., 0., 0.);
    let init_vel = Velocity(vec2(0.5, 1.).normalize() * BALL_SPEED);

    commands.spawn((ball, ball_mat, transform, init_vel, Ball));
}

fn make_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ball = Mesh2d(meshes.add(Rectangle::new(BALL_SIZE, BALL_SIZE * 10.)));
    let ball_mat = MeshMaterial2d(materials.add(Color::from(basic::WHITE)));
    let transform = Transform::from_xyz(350., 0., 0.);
    let init_vel = Velocity(vec2(0., 0.));

    commands.spawn((ball, ball_mat, transform, init_vel, Ball));
}

fn make_cpu_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ball = Mesh2d(meshes.add(Rectangle::new(BALL_SIZE, BALL_SIZE * 10.)));
    let ball_mat = MeshMaterial2d(materials.add(Color::from(basic::WHITE)));
    let transform = Transform::from_xyz(-350., 0., 0.);
    let init_vel = Velocity(vec2(0., 0.));

    commands.spawn((ball, ball_mat, transform, init_vel, Ball));
}
