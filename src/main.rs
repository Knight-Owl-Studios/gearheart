mod assets;
mod camera;
mod debug;
mod map;
mod minions;
mod states;
mod towers;

use bevy::prelude::*;

use assets::AssetsPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use map::MapPlugin;
use minions::dragon::DragonPlugin;
use minions::health::HealthPlugin;
use minions::movement::MovementPlugin;
use minions::MinionPlugin;
use states::StatePlugin;
use towers::ammunition::AmmunitionPlugin;
use towers::bolttower::BoltTowerPlugin;
use towers::range::RangePlugin;
use towers::targeting::TargetingPlugin;
use towers::wizardtower::WizardTowerPlugin;
use towers::TowerPlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 50.0,
        })
        .add_plugins(AssetsPlugin)
        .add_plugins(AmmunitionPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(BoltTowerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DragonPlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(WizardTowerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(MinionPlugin)
        .add_plugins(TargetingPlugin)
        .add_plugins(TowerPlugin)
        .add_plugins(RangePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(MapPlugin)
        .add_plugins(DebugPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
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
