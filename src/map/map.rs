use bevy::prelude::*;

use crate::map::asset::MapAssets;
use crate::states::MyStates;

#[derive(Component)]
pub struct Map;

#[derive(Resource, Debug)]
pub struct Path {
    pub points: Vec<Vec3>,
}

#[derive(Resource, Debug)]
pub struct Placements {
    pub points: Vec<Vec3>,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyStates::Game), load_map)
            .add_systems(
                Update,
                (
                    load_path
                        .run_if(|res: Option<Res<Path>>| res.is_none())
                        .run_if(in_state(MyStates::Game)),
                    load_placements
                        .run_if(|res: Option<Res<Placements>>| res.is_none())
                        .run_if(in_state(MyStates::Game)),
                ),
            );
    }
}

fn load_map(mut commands: Commands, map_assets: Res<MapAssets>) {
    commands.spawn((
        SceneBundle {
            scene: map_assets.level.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        },
        Map,
    ));
}

fn load_path(
    mut commands: Commands,
    path_query: Query<(&Children, &Name)>,
    transform_query: Query<&Transform>,
) {
    let mut points = Vec::new();
    for (children, name) in path_query.iter() {
        if name.as_str() == "Path" {
            // iterate over child nodes and add the transforms to the points vector
            for &child in children.iter() {
                if let Ok(transform) = transform_query.get(child) {
                    points.push(transform.translation);
                }
            }
        }
    }
    if points.is_empty() {
        return;
    }
    info!("Path loaded: {:?}", points);
    commands.insert_resource(Path { points });
}

fn load_placements(
    mut commands: Commands,
    placement_query: Query<(&Children, &Name)>,
    transform_query: Query<&Transform>,
) {
    let mut points = Vec::new();
    for (children, name) in placement_query.iter() {
        if name.as_str() == "Placements" {
            // iterate over child nodes and add the transforms to the points vector
            for &child in children.iter() {
                if let Ok(transform) = transform_query.get(child) {
                    points.push(transform.translation);
                }
            }
        }
    }
    if points.is_empty() {
        return;
    }
    info!("Placements loaded: {:?}", points);
    commands.insert_resource(Placements { points });
}
