use bevy::prelude::*;
use bevy_egui::{egui, egui::*, EguiContexts};
use crate::game_state::GameState;


/// Плагин отвечающий за отображение и логику основного меню
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_update_main_main_menu.run_if(in_state(GameState::MainMenu)));
    }
}

/// Вызывается каждый кадр при нахождении в состоянии [GameState::MainMenu]
fn on_update_main_main_menu(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let ctx = contexts.ctx_mut();
    egui::Window::new("Main menu")
        // Выставляем меню по центру экрана
        .anchor(Align2::CENTER_CENTER, vec2(0., 0.))

        .resizable(false)
        .collapsible(false)
        .title_bar(false)

        // Заменяем стиль окна на стиль панели
        .frame(Frame::central_panel(&ctx.style()))

        .show(contexts.ctx_mut(), |ui| {
            let single = ui.button("Single");
            if single.clicked() {
                game_state.set(GameState::Game)
            }
            ui.button("Multiplayer (WIP)")
        });
}
