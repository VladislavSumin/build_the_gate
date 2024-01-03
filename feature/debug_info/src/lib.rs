use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// Плагин создает окошко с debug информацией, такой как fps и так далее.
pub struct DebugInfoRenderPlugin;

impl Plugin for DebugInfoRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_fps_system);
    }
}

/// Создает окошко с статистикой FPS
fn debug_fps_system(
    mut contexts: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
) {
    let fps_diagnostics = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS);

    match fps_diagnostics {
        None => { return; }
        Some(fps) => {
            let current_fps = fps.value().unwrap_or_default() as u32;
            let smoothed_fps = fps.smoothed().unwrap_or_default() as u32;
            let average_fps = fps.average().unwrap_or_default() as u32;
            let min_fps = fps.values()
                .map(|x| { *x })
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default() as u32;

            egui::Window::new("FPS").show(contexts.ctx_mut(), |ui| {
                ui.label(format!("current {}", current_fps));
                ui.label(format!("smoothed {}", smoothed_fps));
                ui.label(format!("average {}", average_fps));
                ui.label(format!("min (per last 20 frame) {}", min_fps));
            });
        }
    }
}