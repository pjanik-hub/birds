pub mod bird;
pub mod debug;
pub mod movement;

use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

use bird::{Bird, BirdPlugin};
use debug::DebugPlugin;
use movement::{MovementPlugin, Velocity};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin,
        MovementPlugin,
        BirdPlugin,
        DebugPlugin,
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, handle_keypress)
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn(
        TextBundle::from_section("Press space to toggle wireframes", TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
    );
}

fn handle_keypress(
    #[cfg(not(target_arch = "wasm32"))] mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Bird>>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        for mut velocity in query.iter_mut() {
            let addition_vel = velocity.value.normalize() * 50.0;
            velocity.value += addition_vel;
        }
    }
}
