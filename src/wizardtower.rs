use bevy::prelude::*;
use crate::{range::Range, targeting::Targeting, tower::TowerBundle};

const STARTING_TRANSLATION: Vec3 = Vec3::new(6.507063, -1.3359288, 9.231256);
const STARTING_TRANSLATION_2: Vec3 = Vec3::new(-5.943249, -1.3359288, 9.456453);

#[derive(Component)]
pub struct WizardTower {}

pub struct WizardTowerPlugin;

impl Plugin for WizardTowerPlugin {
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
                scene: asset_server.load("Wizrd Tower.glb#Scene0"),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
            range: Range { min: 0.0, max: 6.0 },
            targeting: Targeting { target: None },
            ..Default::default()
        },
        WizardTower {},
    ));
    
    commands.spawn((
        TowerBundle {
            model: SceneBundle {
                scene: asset_server.load("Wizrd Tower.glb#Scene0"),
                transform: Transform::from_translation(STARTING_TRANSLATION_2),
                ..default()
            },
            range: Range { min: 0.0, max: 6.0 },
            targeting: Targeting { target: None },
            ..Default::default()
        },
        WizardTower {},
    ));

}
