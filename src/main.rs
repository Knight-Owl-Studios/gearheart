use bevy::prelude::*;

fn main() {
    App::new()
    // Bevy built-ins.
    .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
    .insert_resource(AmbientLight {
        color: Color::default(),
        brightness: 1,
    })
    .add_plugins(DefaultPlugins)
    // User defined plugins.
    // .add_plugins(DebugPlugin)
    .run();

}
