use bevy::prelude::States;

/// Основное состояние игры
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
}