use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const BALL_SIZE: f32 = 5.;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    position: Position,
}

impl BallBundle {
    fn new() -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(0., 0.)),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_ball)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, move_ball)
        // Add our projection system to run after
        // we move our ball so we are not reading
        // movement one frame behind
        .add_systems(Update, project_positions.after(move_ball))
        .run();
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball...");

    let shape = Mesh::from(Circle::new(BALL_SIZE));
    let color = ColorMaterial::from(Color::srgb(1., 0., 0.));

    // `Assets::add` will load these into memory and return a
    // `Handle` (an ID) to these assets. When all references
    // to this `Handle` are cleaned up the asset is cleaned up.
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    // Here we are using `spawn` instead of `spawn_empty`
    // followed by an `insert`. They mean the same thing,
    // letting us spawn many components on a new entity at once.
    commands.spawn((
        BallBundle::new(),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}
fn move_ball(
    // Give me all positions that also contain a `Ball` component
    mut ball: Query<&mut Position, With<Ball>>,
) {
    let mut position = ball.get_single_mut().expect("needs a single");
    position.0.x += 1.0;
}
