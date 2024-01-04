mod camera;
mod main_menu;
mod game_state;
mod network;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use debug_info::DebugInfoRenderPlugin;
use key_bindings::KeyBindingsPlugin;
use crate::camera::CameraPlugin;
use crate::game_state::{GameMode, GameState};
use crate::main_menu::MainMenuPlugin;
use crate::network::NetworkPlugin;

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
        .add_plugins(NetworkPlugin)

        // Инициализация состояний
        .add_state::<GameState>()
        .add_state::<GameMode>()

        // Тестовые системы из main файла (временные)
        .add_systems(OnEnter(GameState::Game), setup_game_state)
        .add_systems(OnExit(GameState::Game), cleanup_game_state)

        .run();
}

/// Маркер тестового куба, нужен для возможности удалить тестовый куб
#[derive(Component)]
struct TestCube;

fn setup_game_state(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Спавним тестовый куб что бы была точка ориентира в мире
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(1.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        TestCube,
    ));
}

fn cleanup_game_state(
    mut commands: Commands,
    test_cube_query: Query<Entity, With<TestCube>>,
) {
    let test_cube = test_cube_query.single();
    commands.entity(test_cube).despawn();
}