use bevy::prelude::*;
use crate::{range::Range, targeting::Targeting, tower::TowerBundle};

const STARTING_TRANSLATION: Vec3 = Vec3::new(-6.1691093, -1.1100681, -7.9348316);
const STARTING_TRANSLATION_2: Vec3 = Vec3::new(6.4087896, -1.1100681, -7.9131517);

#[derive(Component)]
pub struct BoltTower {}

pub struct BoltTowerPlugin;

impl Plugin for BoltTowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_tower);
    }
}

// fn spin(
//     mut query: Query<(&Name, &mut Transform, &target)>,
//     targetQuery: Query<&Transform>,
// ) {


//     let Ok(dragon_transform) = targetQuery.get_mut() else {
//         return;
//     };

//     for (name, mut transform) in query.iter_mut() {
//         if name.as_str() == "RotationCylinder" { // Replace with your object's name
//             transform.look_at(-dragon_transform.translation, Vec3::Y);
//         }
//     }
// }

fn spawn_tower(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TowerBundle {
            model: SceneBundle {
                scene: asset_server.load("Bolt_Tower.glb#Scene0"),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
            range: Range { min: 0.0, max: 30.0 },
            targeting: Targeting { target: None },
        },
        BoltTower {},
    ));
    commands.spawn((
        TowerBundle {
            model: SceneBundle {
                scene: asset_server.load("Bolt_Tower.glb#Scene0"),
                transform: Transform::from_translation(STARTING_TRANSLATION_2),
                ..default()
            },
            range: Range { min: 0.0, max: 30.0 },
            targeting: Targeting { target: None },
        },
        BoltTower {},
    ));

}
