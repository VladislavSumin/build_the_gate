mod camera;
mod key_binding;
mod main_menu;
mod game_state;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use debug_info::DebugInfoRenderPlugin;
use crate::camera::CameraPlugin;
use crate::game_state::GameState;
use crate::key_binding::KeyBindingsPlugin;
use crate::main_menu::MainMenuPlugin;

fn main() {
    App::new()
        // Встроенные в bevy плагины
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)

        // Сторонние плагины
        .add_plugins(EguiPlugin)

        // Внутренние плагины
        .add_plugins(DebugInfoRenderPlugin)
        .add_plugins(KeyBindingsPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(CameraPlugin)

        // Инициализация состояний
        .add_state::<GameState>()

        // Тестовые системы из main файла (временные)
        .add_systems(OnEnter(GameState::Game), setup_game_state)

        .run();
}

fn setup_game_state(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Спавним тестовый куб что бы была точка ориентира в мире
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::new(1.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}