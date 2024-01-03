use bevy::prelude::States;

/// Основное состояние игры
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
}

/// Режим игры
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug, States)]
pub enum GameMode {
    /// Игра не запущена, режим не определен
    #[default]
    None,
    Single,
    Multiplayer,
}