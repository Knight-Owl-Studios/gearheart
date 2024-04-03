use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MyStates {
    #[default]
    AssetLoading,
    MainMenu,
    Game,
    Paused,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MyStates>();
    }
}
