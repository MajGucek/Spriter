#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]


mod constants;
mod sprite_format;
mod input_string;

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy_egui::*;
use egui::{FontId, Frame, Pos2};
use crate::constants::*;
use crate::input_string::InputString;
use crate::sprite_format::SpriteType;

#[derive(Resource)]
struct FileName(String);


#[derive(Resource, Eq, PartialEq)]
enum EditorStep {
    None,
    FileCreated,
    SettingsSet,
}

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
enum EditorState {
    #[default]
    None,
    Show,
}

#[derive(Resource)]
struct SpriteInput(InputString);

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
        .insert_resource(SpriteInput(InputString::default()))
        .insert_state(EditorState::None)
        .insert_resource(EditorStep::None)
        .insert_resource(SpriteType::default())
        .insert_resource(FileName("".to_owned()))
        .add_plugins(EguiPlugin)
        .add_systems(Update, (
                main_window.run_if(in_state(EditorState::None)),
                settings_window,
                editor_window.run_if(in_state(EditorState::Show))

            )
        )
        .run();
}



fn editor_window(
    mut egui_ctx: EguiContexts,
    mut sprite: ResMut<SpriteType>,
    mut sprite_input: ResMut<SpriteInput>,
) {
    let gui = egui::Window::new("Editor")
        .title_bar(true)
        .resizable(true)
        .movable(true)
        .default_pos(Pos2::new(200.0, 0.0))
        .frame(Frame {
            fill: MENU_BG,
            ..default()
        });

    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.add(
            egui::TextEdit::singleline(&mut sprite_input.0)
                .desired_rows(sprite.height.value as usize)
                .desired_width((sprite.width.value * 20) as f32)
        );
        println!("{:?}", &sprite_input.0);
    });
}


fn settings_window(
    mut egui_ctx: EguiContexts,
    file_name: Res<FileName>,
    mut sprite: ResMut<SpriteType>,
    mut editor_step: ResMut<EditorStep>,
    mut next: ResMut<NextState<EditorState>>,
    mut sprite_input: ResMut<SpriteInput>,
) {
    if *editor_step == EditorStep::None { return; }
    let gui = egui::Window::new("Sprite settings")
        .title_bar(true)
        .resizable(true)
        .movable(true)
        .default_pos(Pos2::new(0.0, 0.0))
        .frame(Frame {
            fill: MENU_BG,
            ..default()
        });
    let mut apply_clicked = false;
    gui.show(egui_ctx.ctx_mut(), |ui| {
        ui.style_mut()
            .override_font_id = Some(FontId::new(
            FONT_SIZE,
                FONT,
        ));


        if *editor_step == EditorStep::FileCreated {
            ui.horizontal(|ui| {
                ui.label("Width");
                ui.text_edit_singleline(&mut sprite.width);
            });

            ui.horizontal(|ui| {
                ui.label("Height");
                ui.text_edit_singleline(&mut sprite.height);
            });

            ui.horizontal(|ui| {
                let apply = ui.add(
                    egui::Button::new("Apply")
                        .rounding(2.0)
                );

                ui.label(
                    egui::RichText::new("You can only do this once!")
                        .size(FONT_SIZE / 2.)
                        .color(FONT_COLOR)
                        .monospace()
                );

                if apply.clicked() {
                    apply_clicked = true;
                }
            });
        }


        if *editor_step == EditorStep::SettingsSet {
            ui.label(
                egui::RichText::new(format!("{}.guspr", file_name.0))
                    .size(FONT_SIZE * 1.5)
                    .heading()
            );

            ui.horizontal(|ui| {
                ui.label("Add frame");
                let add = ui.button("+");
                if add.clicked() {
                    next.set(EditorState::Show);
                    sprite.add_frame();
                    let new_ind = sprite.ind.unwrap_or(0).clone().saturating_add(1);
                    sprite.move_ind(new_ind);
                }
            });
            match sprite.ind {
                None => {ui.label("No frames yet"); }
                Some(ok) => {
                    ui.label(format!("Index: {:?}", ok));
                    ui.horizontal(|ui| {
                        ui.separator();
                        let move_left = ui.button("<");
                        ui.separator();
                        let move_right = ui.button(">");

                        if move_left.clicked() {
                            let new_ind = ok.saturating_sub(1).clone();
                            sprite.move_ind(new_ind);
                        }

                        if move_right.clicked() {
                            let new_ind = ok.saturating_add(1).clone();
                            sprite.move_ind(new_ind);
                        }
                    });
                }
            }

        }

        if apply_clicked {
            *editor_step = EditorStep::SettingsSet;
            sprite_input.0.width = sprite.width.value.clone();
            sprite_input.0.height = sprite.height.value.clone();
        }
    });
}



fn main_window(
    mut egui_ctx: EguiContexts,
    mut file_name: ResMut<FileName>,
    mut editor_step: ResMut<EditorStep>,
) {
    if *editor_step != EditorStep::None {
        return;
    }

    let gui = egui::Window::new("")
        .title_bar(false)
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
            FONT_SIZE,
            FONT
        ));
        ui.vertical_centered(|ui| {
            ui.heading("Tool for creating sprites");
            ui.hyperlink_to("Party Games", "https://github.com/MajGucek/PartyGames");
            ui.hyperlink_to("Author", "https://github.com/MajGucek");
            ui.separator();
            ui.add(
                egui::TextEdit::singleline(&mut file_name.0).hint_text("name ...")
            )
        });
        let save = ui.button("Create");
        let load = ui.button("Load");

        if save.clicked() {
            if file_name.0 != "" {
                println!("Creating: {}", file_name.0);
                //let file = File::create(format!("{}.guspr", file_name.0));
                *editor_step = EditorStep::FileCreated;
            }
        }

        if load.clicked() {
            if file_name.0 != "" {
                println!("Loading: {}", file_name.0);
                *editor_step = EditorStep::FileCreated;
                todo!();
            }
        }
    });

}
