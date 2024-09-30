pub mod bird;
pub mod movement;

use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bird::BirdPlugin;
use movement::MovementPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin,
        MovementPlugin,
        BirdPlugin,
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

#[cfg(not(target_arch = "wasm32"))]
fn handle_keypress(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
