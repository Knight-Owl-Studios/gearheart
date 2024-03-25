use bevy::prelude::*;


const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 4.0);

#[derive(Bundle)]
struct BoltTowerBundle {
    model: SceneBundle,
}
#[derive(Component)]
pub struct BoltTower {}

pub struct BoltTowerPlugin;

impl Plugin for BoltTowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_tower)
            .add_systems(Update, spin);
    }
}

fn spin(
    mut query: Query<(&Name, &mut Transform)>,
    time: Res<Time>
) {
    for (name, mut transform) in query.iter_mut() {
        info!(
            "Name {:?}",
            name,
        );
        
        if name.as_str() == "RotationCylinder" || name.as_str() == "BoltBarrel" || name.as_str() == "PressureTankBottom" || name.as_str() == "PressureTankTop" || name.as_str() == "Roof" || name.as_str() == "Tower" { // Replace with your object's name
            transform.rotate_y(0.5 * time.delta_seconds()); // Adjust rotation as needed
        }
    }
}

fn spawn_tower(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        BoltTowerBundle {
            model: SceneBundle {
                scene: asset_server.load("Bolt_Tower.glb#Scene0"),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        BoltTower {},
    ));

}
