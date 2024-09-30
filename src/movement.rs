use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub model: MaterialMesh2dBundle<ColorMaterial>,
}

fn update_entity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

fn update_velocity(windows: Query<&Window>, mut query: Query<(&mut Velocity, &Transform)>) {
    let window = windows.single();
    let width = window.width();
    let height = window.height();

    for (mut velocity, transform) in &mut query.iter_mut() {
        if transform.translation.x > width / 2.0 {
            velocity.value = Vec3::new(-velocity.value.x, velocity.value.y, 0.0);
        }
        if transform.translation.x < -width / 2.0 {
            velocity.value = Vec3::new(-velocity.value.x, velocity.value.y, 0.0);
        }
        if transform.translation.y > height / 2.0 {
            velocity.value = Vec3::new(velocity.value.x, -velocity.value.y, 0.0);
        }
        if transform.translation.y < -height / 2.0 {
            velocity.value = Vec3::new(velocity.value.x, -velocity.value.y, 0.0);
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_velocity, update_entity).chain());
    }
}
