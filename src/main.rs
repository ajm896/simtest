use std::f32::consts::TAU;

use bevy::color::palettes::basic;
use bevy::prelude::*;
use rand::{rng, Rng};

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
#[derive(Component)]
struct Collider;

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;
const BALL_SIZE: f32 = 10.;
const BALL_SPEED: f32 = 5.;

const PADDLE_WIDTH: f32 = BALL_SIZE;
const PADDLE_HEIGHT: f32 = BALL_SIZE * 10.;

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
    app.add_systems(Update, (move_ball, move_player));

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn move_ball(
    mut gizmos: Gizmos,
    mut ball: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    paddles: Query<(&Collider, &Transform), Without<Ball>>,
) {
    for (mut velocity, mut transform) in &mut ball {
        // Move ball
        transform.translation += velocity.0.extend(0.);

        // helper
        gizmos.ray(
            transform.translation,
            velocity.0.extend(0.) * 10.,
            basic::LIME,
        );

        // Wall Collison
        if transform.translation.x >= WINDOW_WIDTH / 2. - 5. {
            velocity.0 = velocity.0.reflect(Vec2::NEG_X);
        } else if transform.translation.y >= WINDOW_HEIGHT / 2. - 5. {
            velocity.0 = velocity.0.reflect(Vec2::NEG_Y);
        } else if transform.translation.x <= -WINDOW_WIDTH / 2. + 5. {
            velocity.0 = velocity.0.reflect(Vec2::NEG_X);
        } else if transform.translation.y <= -WINDOW_HEIGHT / 2. + 5. {
            velocity.0 = velocity.0.reflect(Vec2::NEG_Y);
        }
        // Paddle Collison
        for (_paddle_coll, paddle_loc) in paddles {
            let ball_x = transform.translation.x;
            let ball_y = transform.translation.y;
            let paddle_x = paddle_loc.translation.x;
            let paddle_y = paddle_loc.translation.y;

            let paddle_top = paddle_y + PADDLE_HEIGHT / 2.;
            let paddle_bottom = paddle_y - PADDLE_HEIGHT / 2.;

            let ball_top = ball_y + BALL_SIZE / 2.;
            let ball_bottom = ball_y - BALL_SIZE / 2.;

            let collision_x = ball_x == paddle_x;
            let collision_y = ball_top < paddle_top && ball_bottom > paddle_bottom;

            if collision_x && collision_y {
                velocity.0 = -velocity.0
            }
        }
    }
}
fn move_player(input: Res<ButtonInput<KeyCode>>, mut player: Single<&mut Transform, With<Player>>) {
    if input.pressed(KeyCode::ArrowUp) {
        player.translation.y += BALL_SPEED;
    }
    
    if input.pressed(KeyCode::ArrowDown) {
        player.translation.y -= BALL_SPEED;
    }
}

fn make_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rng();
    let shape = Rectangle::new(BALL_SIZE, BALL_SIZE);
    let ball = Mesh2d(meshes.add(shape));
    let ball_mat = MeshMaterial2d(materials.add(Color::from(basic::WHITE)));
    let transform = Transform::from_xyz(0., 0., 0.);
    let direction: Vec2 = vec2(rng.random_range(0.0..TAU), rng.random_range(0.0..TAU));
    let init_vel = Velocity(BALL_SPEED * direction.normalize());

    commands.spawn((ball, ball_mat, transform, init_vel, Ball));
}

fn make_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);
    let paddle = Mesh2d(meshes.add(shape));
    let paddle_mat = MeshMaterial2d(materials.add(Color::from(basic::WHITE)));
    let transform = Transform::from_xyz(350., 0., 0.);
    let init_vel = Velocity(Vec2::ZERO);

    commands.spawn((Player,paddle, paddle_mat, transform, init_vel, Paddle, Collider));
}

fn make_cpu_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);
    let paddle = Mesh2d(meshes.add(shape));
    let paddle_mat = MeshMaterial2d(materials.add(Color::from(basic::WHITE)));
    let transform = Transform::from_xyz(-350., 0., 0.);
    let init_vel = Velocity(Vec2::ZERO);

    commands.spawn((paddle, paddle_mat, transform, init_vel, Paddle, Collider));
}
