use bevy::{prelude::*, render::camera::ScalingMode};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn(Camera3dBundle {
    projection: OrthographicProjection {
        // 6 world units per window height.
        scaling_mode: ScalingMode::FixedVertical(6.0),
        ..default()
    }
    .into(),
    transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}
