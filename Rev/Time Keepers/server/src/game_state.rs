use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Preload,
    MainMenu,
    Playing,
    Settings,
    Paused,
}
