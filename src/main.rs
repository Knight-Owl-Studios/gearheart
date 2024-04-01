mod bolttower;
mod wizardtower;
mod camera;
mod debug;
mod dragon;
mod movement;
mod tower;
mod targeting;
mod minion;
mod range;
mod map;

use bevy::prelude::*;
use bolttower::BoltTowerPlugin;
use wizardtower::WizardTowerPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use dragon::DragonPlugin;
use targeting::TargetingPlugin;
use movement::MovementPlugin;
use minion::MinionPlugin;
use range::RangePlugin;
use map::MapPlugin;

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
    .add_plugins(DragonPlugin)
    .add_plugins(WizardTowerPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(MinionPlugin)
    .add_plugins(TargetingPlugin)
    .add_plugins(RangePlugin)
    .add_plugins(MapPlugin)
    .add_plugins(DebugPlugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(9.0, 15.0, 6.0),
        ..default()
    });
}
