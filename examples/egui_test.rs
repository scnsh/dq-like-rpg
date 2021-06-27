use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui_example.system())
        .run();
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx(), |ui| {
        ui.label("world");
    });
}
