use bevy::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 4.0);

#[derive(Bundle)]
struct BoltTowerBundle {
    model: SceneBundle,
}

pub struct BoltTowerPlugin;

impl Plugin for BoltTowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tower);
    }
}

fn spawn_tower(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(BoltTowerBundle {
        model: SceneBundle {
            scene: asset_server.load("Bolt-Tower.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}
