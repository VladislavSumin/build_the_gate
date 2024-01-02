mod camera;
mod key_binding;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::camera::CameraPlugin;
use crate::key_binding::KeyBindingsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(KeyBindingsPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system)
        .run();
}

fn ui_example_system(
    mut contexts: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.value())
        .unwrap_or(0.) as u32;

    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("FPS {}", fps));
    });
}

fn setup(
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