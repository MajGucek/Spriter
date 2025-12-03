mod constants;
mod sprite_format;

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy_egui::*;
use egui::{FontId, Frame, Pos2};
use crate::constants::*;


#[derive(Resource)]
struct FileName(String, bool);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::Windowed,
                position: WindowPosition::At(IVec2 { x: 0, y: 40}),
                title: "Spriter".to_owned(),
                ..default()
            }),
            ..default()
        }).set(RenderPlugin {
            render_creation: bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(sprite_format::Sprite::default())
        .insert_resource((FileName("".to_owned(), false)))
        .add_plugins(EguiPlugin)
        .add_systems(Update, (
                main_window,
                editor_window
            )
        )
        .run();
}


fn editor_window(
    mut egui_ctx: EguiContexts,
    file_name: Res<FileName>,
    mut sprite: ResMut<sprite_format::Sprite>,
) {
    let gui = egui::Window::new("Sprite settings")
        .title_bar(true)
        .resizable(true)
        .movable(true)
        .default_pos(Pos2::new(400.0, 0.0))
        .frame(Frame {
            fill: MENU_BG,
            ..default()
        });

    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.style_mut()
            .override_font_id = Some(FontId::new(
            14.0,
                egui::FontFamily::Monospace,
        ));
        if file_name.1 == false {
            ui.heading("Waiting for File creating/loading!");
            return;
        }

        ui.heading("Editor");
        ui.horizontal(|ui| {
            ui.label("Width");
            ui.text_edit_singleline(&mut sprite.width);
        });

        ui.horizontal(|ui| {
            ui.label("Height");
            ui.text_edit_singleline(&mut sprite.height);
        });

        ui.horizontal(|ui| {
            ui.label("Add frame");
            let add = ui.button("+");
            if add.clicked() {
                sprite.add_frame();
                let new_ind = sprite.ind.clone().saturating_add(1);
                sprite.move_ind(new_ind);
            }
        });

        ui.label(format!("Index: {}", sprite.ind));

        ui.horizontal(|ui| {
            ui.separator();
            let move_left = ui.button("<");
            ui.separator();
            let move_right = ui.button(">");

            if move_left.clicked() {
                let new_ind = sprite.ind.saturating_sub(1).clone();
                sprite.move_ind(new_ind);
            }

            if move_right.clicked() {
                let new_ind = sprite.ind.saturating_add(1).clone();
                sprite.move_ind(new_ind);
            }

        });



    });
}



fn main_window(
    mut egui_ctx: EguiContexts,
    mut file_name: ResMut<FileName>,
) {
    let gui = egui::Window::new("Main")
        .title_bar(true)
        .resizable(true)
        .movable(true)
        .default_pos(Pos2::new(0.0, 0.0))
        .frame(Frame {
            fill: MENU_BG,
            ..default()
        });
    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.style_mut()
            .override_font_id = Some(FontId::new(
            16.0,
            egui::FontFamily::Monospace
        ));
        ui.vertical_centered(|ui| {
            ui.heading("Tool for creating sprites");
            ui.hyperlink_to("Party Games", "https://github.com/MajGucek/PartyGames");
            ui.hyperlink_to("Author", "https://github.com/MajGucek");
            ui.separator();
            ui.add(
                egui::TextEdit::singleline(&mut file_name.0).hint_text("file name ...")
            )
        });
        let save = ui.button("Create");
        let load = ui.button("Load");

        if save.clicked() {
            if file_name.0 != "" {
                println!("Creating: {}", file_name.0);
                //let file = File::create(format!("{}.guspr", file_name.0));
                file_name.1 = true;
            }
        }

        if load.clicked() {
            if file_name.0 != "" {
                println!("Loading: {}", file_name.0);
                file_name.1 = true;
                todo!();
            }
        }
    });

}
