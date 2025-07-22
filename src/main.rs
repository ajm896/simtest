use bevy::prelude::*;
use bevy::color::palettes::basic;
fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, setup);
    app.add_systems(Update, move_circle);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
    ) 
{
    commands.spawn(Camera2d);
    let circle = Mesh2d(meshes.add(Circle::new(100.)));
    let cir_mat = MeshMaterial2d(materials.add(Color::from(basic::PURPLE)));
    let transform = Transform::from_xyz(0., 0., 0.);

    commands.spawn((circle, cir_mat, transform));
}

fn move_circle(
    mut circle: Query<(&Mesh2d, &mut Transform)>,
)
{
    for (_, mut transform) in &mut circle {
        transform.translation.x += 10.;
    }
}
