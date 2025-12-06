#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]


mod constants;
mod sprite_format;

use std::time::Duration;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::time::common_conditions::on_timer;
use bevy_egui::*;
use egui::{Align, Color32, FontId, FontSelection, Frame, Margin, Pos2, Rounding, Stroke};
use crate::constants::*;
use crate::sprite_format::{Sprite, SpriteFrame, TerminalChar};

#[derive(Resource)]
struct FileName(String);



#[derive(Resource, Default)]
struct InputField {
    width: u16,
    height: u16,
    rows: Vec<String>,
}


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
        .insert_resource(InputField::default())
        .insert_state(EditorState::None)
        .insert_resource(EditorStep::None)
        .insert_resource(Sprite::default())
        .insert_resource(FileName("".to_owned()))
        .add_plugins(EguiPlugin)
        .add_systems(Update, (
                main_window.run_if(in_state(EditorState::None)),
                settings_window,
                editor_window.run_if(in_state(EditorState::Show)),
                writer.run_if(in_state(EditorState::Show)).run_if(on_timer(Duration::from_secs_f32(1.0 / 1.0)))
            )
        )
        .add_systems(OnEnter(EditorState::Show), check_settings)
        .run();
}



fn writer(
    mut sprite: ResMut<Sprite>,
    input_string: ResMut<InputField>,
) {
    let ind = sprite.ind.unwrap().clone() as usize;
    let empty_char = String::from(" ");

    let mut sprite_frame = SpriteFrame::default();
    for row in 0..input_string.rows.len() {
        let mut hor: Vec<TerminalChar> = Vec::new();


        for ch in input_string.rows.get(row).unwrap_or(&empty_char).chars() {
            hor.push(TerminalChar::from_char(ch));
        }

        sprite_frame.frame.push(hor);
    }

    if let Some(el) = sprite.data.get_mut(ind) {
        *el = sprite_frame;
    }
}


fn editor_window(
    mut egui_ctx: EguiContexts,
    mut sprite: ResMut<Sprite>,
    mut input_string: ResMut<InputField>,
) {
    let gui = egui::Window::new("Editor")
        .title_bar(true)
        .min_size(egui::vec2(300.0, 550.0))
        .default_size(egui::vec2(400.0, 600.0))
        .default_pos(Pos2::new(200.0, 0.0))
        .frame(Frame {
            fill: MENU_BG,
            ..default()
        });

    gui.show(egui_ctx.ctx_mut(), |ui| {
        Frame::default()
            .fill(Color32::BLACK)
            .stroke(Stroke::new(2.0, Color32::WHITE))
            .rounding(Rounding::same(0.0))
            .inner_margin(Margin::same(3.0))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    let width = input_string.width.clone();
                    for row in 0..input_string.height {
                        //let id = ui.make_persistent_id(format!("row_{}", row));
                        //let response =
                            ui.scope(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut input_string.rows[row as usize])
                                    //.id(id)
                                    .frame(false)
                                    .char_limit(width as usize)
                                    .horizontal_align(Align::LEFT)
                                    .desired_width(ui.available_width())
                                    .font(FontSelection::from(FontId::new(FONT_SIZE * 2., FONT)))
                            );
                        })
                                //.response
                        ;

                        /* This just doesn't work, maybe try GPT
                        if response.has_focus() {
                            input_string.focused_row = row;

                            if response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) && row > 0 {
                                let id = ui.make_persistent_id(format!("row_{}", row - 1));
                                ui.memory_mut(|m| m.request_focus(id));
                                input_string.focused_row = row - 1;
                            }

                            if response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::Enter)) && row  + 1 < input_string.height {
                                let id = ui.make_persistent_id(format!("row_{}", row + 1));
                                ui.memory_mut(|m| m.request_focus(id));
                                input_string.focused_row = row + 1;
                            }
                        }
                        */

                    }
                });
            });

    });
}


fn settings_window(
    mut egui_ctx: EguiContexts,
    file_name: Res<FileName>,
    mut sprite: ResMut<Sprite>,
    mut editor_step: ResMut<EditorStep>,
    mut next: ResMut<NextState<EditorState>>,
    mut input_string: ResMut<InputField>,
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
                    //let new_ind = sprite.ind.unwrap_or(0).clone().saturating_add(1);
                    //sprite.move_ind(new_ind);
                    trigger_reload(&mut input_string, sprite.ind.unwrap(), &mut sprite);
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
                            match sprite.move_ind(new_ind) {
                                Ok(_) => trigger_reload(&mut input_string, new_ind, &mut sprite),
                                Err(e) => println!("Cannot move index: {:?}", e)
                            }
                        }

                        if move_right.clicked() {
                            let new_ind = ok.saturating_add(1).clone();
                            match sprite.move_ind(new_ind) {
                                Ok(_) => trigger_reload(&mut input_string, new_ind, &mut sprite),
                                Err(e) => println!("Cannot move index: {:?}", e)
                            }
                        }
                    });
                }
            }

        }

        if apply_clicked {
            *editor_step = EditorStep::SettingsSet;
            input_string.width = sprite.width.value.clone();
            input_string.height = sprite.height.value.clone();
            input_string.rows = vec![String::new(); input_string.height as usize];
        }
    });
}



fn trigger_reload(
    input_field: &mut ResMut<InputField>,
    ind: u16,
    sprite: &mut ResMut<Sprite>,
) {
    println!("Loading frame with index: {:?}", ind);
    println!("Loaded: {:?}", sprite.data);
    let mut frame = &sprite.data.get(ind as usize).unwrap().frame;


    let mut new_input_field = InputField::default();
    new_input_field.height = sprite.height.value;
    new_input_field.width = sprite.width.value;

    if frame.is_empty() {
        for row in 0..new_input_field.height {
            new_input_field.rows.push(String::new());
        }
    } else {
        for row in frame.iter() {
            let mut str = String::new();
            for ch in row.iter() {
                str.push(ch.char as char)
            }
            new_input_field.rows.push(str);
        }
    }

    **input_field = new_input_field;

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


fn check_settings(
    sprite: ResMut<Sprite>,
    input_string: ResMut<InputField>,
) {
    if sprite.height.value != input_string.height || sprite.width.value != input_string.width {
        panic!("WOW, shouldn't happen, de-sync error!");
    }
}