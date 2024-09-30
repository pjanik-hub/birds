use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::{app::Startup, math::Vec3};

use crate::movement::*;

#[derive(Component, Debug)]
pub struct Bird;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird);
    }
}

const RADIUS: f32 = 50.0;
const STARTING_POS: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const STARTING_VEL: Vec3 = Vec3::new(200.0, 50.0, 0.0);
const STARTING_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

fn spawn_bird(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(STARTING_VEL),
            model: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Mesh::from(Circle::new(RADIUS)))),
                material: materials.add(ColorMaterial::from(STARTING_COLOR)),
                transform: Transform::from_translation(STARTING_POS),
                ..default()
            },
        },
        Bird,
    ));
}
