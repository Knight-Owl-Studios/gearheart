mod bolttower;
mod camera;
mod debug;

use bevy::prelude::*;
use bolttower::BoltTowerPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;


fn main() {
    App::new()
    // Bevy built-ins.
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.15)))
    .insert_resource(AmbientLight {
        color: Color::default(),
        brightness: 50.0,
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(BoltTowerPlugin)
    .add_plugins(CameraPlugin)
    // .add_plugins(DebugPlugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
