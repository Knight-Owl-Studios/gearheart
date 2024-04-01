use bevy::{prelude::*, render::camera::ScalingMode};
// use crate::dragon::Dragon;

#[derive(Component)]
pub struct GameCamera;


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
          .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn((Camera3dBundle {
    projection: OrthographicProjection {
        // 6 world units per window height.
        scaling_mode: ScalingMode::FixedVertical(6.0),
        scale: 4.0,
        ..default()
    }
    .into(),
    transform: Transform::from_xyz(500.0, 500.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  },
  GameCamera));
}

// fn update_camera_system(
//     mut query: Query<&mut Transform, (With<GameCamera>, Without<Dragon>)>,
//     target_query: Query<&Transform, With<Dragon>>,
// ) {
//     let Ok(&target_transform) = target_query.get_single() else {
//         return;
//     };
    
//     let Ok(mut camera_transform) = query.get_single_mut() else {
//         return;
//     };

//     // Update the camera to look at the target's current position
//     camera_transform.look_at(target_transform.translation, Vec3::Y);
// }
