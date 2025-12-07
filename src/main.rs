#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]


mod constants;
mod sprite_format;

use std::{fs, io};
use std::io::{Write};
use std::time::Duration;
use bevy::prelude::*;
use bevy::reflect::List;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::time::common_conditions::on_timer;
use bevy_egui::*;
use egui::{Align, Color32, FontId, FontSelection, Frame, Layout, Margin, Pos2, Rounding, Stroke};
use serde::{Deserialize, Serialize};
use crate::constants::*;
use crate::sprite_format::{Sprite, TerminalChar};

#[derive(Resource)]
struct FileName {
    name: String,
    format: ExportFormat,
}

#[derive(Default, Debug)]
enum ExportFormat {
    #[default]
    JSON,
}

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
enum FileState {
    #[default]
    None,
    Exporting,
    Importing,
}



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


#[derive(Serialize, Deserialize)]
struct JsonFormat {
    frame_count: u16,
    width: u16,
    height: u16,
    sprite: String,
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
        .insert_state(FileState::None)
        .insert_resource(InputField::default())
        .insert_state(EditorState::None)
        .insert_resource(EditorStep::None)
        .insert_resource(Sprite::default())
        .insert_resource(FileName {name: "".to_owned(), format: default()})
        .add_plugins(EguiPlugin)
        .add_systems(Update, (
                main_window.run_if(in_state(EditorState::None)),
                settings_window,
                editor_window.run_if(in_state(EditorState::Show)),
                write_to_sprite.run_if(in_state(EditorState::Show)).run_if(on_timer(Duration::from_secs_f32(1.0 / 2.0)))
            )
        )
        .add_systems(OnEnter(FileState::Exporting), write_to_sprite)
        .add_systems(OnEnter(FileState::Exporting), export_sprite.after(write_to_sprite))
        .add_systems(OnEnter(FileState::Importing), import_sprite)
        .add_systems(OnEnter(EditorState::Show), check_settings)
        .run();
}


fn import_sprite(
    mut input_field: ResMut<InputField>,
    mut sprite: ResMut<Sprite>,
    file: Res<FileName>,
    mut export_state: ResMut<NextState<FileState>>,
    mut editor_state: ResMut<NextState<EditorState>>,
    mut editor_step: ResMut<EditorStep>,
) {
    read_file(format!("{}.{}", file.name, FILE_EXT), &mut sprite).expect("Error");
    input_field.height = sprite.height.value;
    input_field.width = sprite.width.value;
    input_field.rows = sprite.data.frames
        .get(0usize).unwrap().iter()
        .map(|row| {
            row.iter()
                .map(|ch| ch.char as char)
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    sprite.move_ind(0u16).expect("Cannot move to index 0");

    export_state.set(FileState::None);
    editor_state.set(EditorState::Show);
    *editor_step = EditorStep::SettingsSet;
}
fn read_file(file_path: String, sprite: &mut Sprite) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let json: serde_json::Value = serde_json::from_reader(file)?;
    sprite.height.value = match json.get("height") {
        Some(ok) => u16::try_from(ok.as_u64().unwrap_or(0u64)).unwrap_or(0),
        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Missing Field height!"))
    };

    sprite.width.value = match json.get("width") {
        Some(ok) => u16::try_from(ok.as_u64().unwrap_or(0u64)).unwrap_or(0),
        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Missing Field width!"))
    };

    let frame_count = match json.get("frame_count") {
        Some(ok) => u16::try_from(ok.as_u64().unwrap_or(0u64)).unwrap_or(0),
        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Missing Field frame_count!"))
    };

    let encoded_sprite = match json.get("sprite") {
        Some(ok) => String::try_from(ok.as_str().unwrap_or("")).unwrap_or(String::new()),
        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Missing Field sprite!"))
    };

    let data = serde_json::from_str::<Vec<Vec<Vec<u8>>>>(encoded_sprite.trim_matches('"'))?;

    sprite.data.frames = data
        .into_iter()
        .map(|frame| {
            frame
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|ascii| TerminalChar::from_char(ascii as char))
                        .collect()
                })
                .collect()
        })
        .collect();

    sprite.ind = Some(0);
    Ok(())
}


fn is_json_array(string: &str) -> bool {
    string.starts_with("[") && string.ends_with("]")
}


fn write_to_sprite(
    mut sprite: ResMut<Sprite>,
    input_field: Res<InputField>,
) {
    let ind = sprite.ind.unwrap().clone() as usize;
    let empty_char = String::from(" ");

    let mut sprite_frame = Vec::new();
    for row in 0..input_field.rows.len() {
        let mut hor: Vec<TerminalChar> = Vec::new();


        for ch in input_field.rows.get(row).unwrap_or(&empty_char).downcast_ref::<String>().unwrap().chars() {
            hor.push(TerminalChar::from_char(ch));
        }

        sprite_frame.push(hor);
    }

    if let Some(el) = sprite.data.frames.get_mut(ind) {
        *el = sprite_frame;
    }
}


fn editor_window(
    mut egui_ctx: EguiContexts,
    mut input_field: ResMut<InputField>,
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
                    let width = input_field.width.clone();
                    for row in 0..input_field.height {
                        ui.add(
                            egui::TextEdit::singleline(&mut input_field.rows[row as usize])
                                //.id(id)
                                .frame(false)
                                .char_limit(width as usize)
                                .horizontal_align(Align::LEFT)
                                .desired_width(ui.available_width())
                                .font(FontSelection::from(FontId::new(FONT_SIZE * 2., FONT)))
                        );
                    }
                });
            });

    });
}


fn settings_window(
    mut egui_ctx: EguiContexts,
    file: Res<FileName>,
    mut sprite: ResMut<Sprite>,
    mut editor_step: ResMut<EditorStep>,
    mut next: ResMut<NextState<EditorState>>,
    mut input_field: ResMut<InputField>,
    mut export_state: ResMut<NextState<FileState>>,
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
                egui::RichText::new(format!("{}.{}", file.name, FILE_EXT))
                    .size(FONT_SIZE * 1.5)
                    .heading()
            );

            ui.horizontal(|ui| {
                ui.label("Add frame");
                let add = ui.button("+");
                if add.clicked() {
                    next.set(EditorState::Show);
                    sprite.add_frame();
                    trigger_reload(&mut input_field, sprite.ind.unwrap(), &mut sprite);
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
                                Ok(_) => trigger_reload(&mut input_field, new_ind, &mut sprite),
                                Err(_) => {
                                    //println!("Cannot move index");
                                }
                            }
                        }

                        if move_right.clicked() {
                            let new_ind = ok.saturating_add(1).clone();
                            match sprite.move_ind(new_ind) {
                                Ok(_) => trigger_reload(&mut input_field, new_ind, &mut sprite),
                                Err(_) => {
                                    //println!("Cannot move index");
                                }
                            }
                        }
                    });


                    let export = ui.button("Export");

                    if export.clicked() {
                        export_state.set(FileState::Exporting);
                    }
                }
            }

        }

        if apply_clicked {
            *editor_step = EditorStep::SettingsSet;
            input_field.width = sprite.width.value.clone();
            input_field.height = sprite.height.value.clone();
            input_field.rows = vec![String::new(); input_field.height as usize];
        }
    });
}


fn export_sprite(
    sprite: Res<Sprite>,
    file: Res<FileName>,
    mut export_state: ResMut<NextState<FileState>>,
) {
    println!("Exporting: {}.{} in format: {:?}", file.name, FILE_EXT, file.format);
    let file_name = format!("{}.{}", file.name, FILE_EXT);

    #[allow(unreachable_patterns)]
    match file.format {
        ExportFormat::JSON => {
            match export_json(&file_name, sprite.as_ref()) {
                Ok(_) => { println!("JSON Export successful!"); },
                Err(e) => { println!("JSON Export error: {:?}", e); }
            }
        },
        _ => {todo!("Implement this export format: {:?}", file.format);}
    }

    export_state.set(FileState::None);
}


fn export_json(file_name: &str, sprite: &Sprite) -> io::Result<()> {
    let file_name = format!("{}", file_name);
    let serialized = JsonFormat {
        frame_count: sprite.data.frames.len() as u16,
        width: sprite.width.value,
        height: sprite.height.value,
        sprite: format!("{}", sprite.data),
    };

    let mut file = fs::File::create(file_name)?;
    serde_json::to_writer_pretty(&mut file, &serialized)?;
    file.flush()?;

    Ok(())
}



fn trigger_reload(
    input_field: &mut ResMut<InputField>,
    ind: u16,
    sprite: &mut ResMut<Sprite>,
) {
    let mut frame = &sprite.data.frames.get(ind as usize).unwrap();


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
    mut export_state: ResMut<NextState<FileState>>,
) {
    if *editor_step != EditorStep::None {
        return;
    }

    let gui = egui::Window::new("Main")
        .resizable(true)
        .movable(true)
        .default_pos(Pos2::new(5.0, 5.0))
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
                egui::TextEdit::singleline(&mut file_name.name).hint_text("name ...")
            )
        });


        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.horizontal(|ui| {
                let save = ui.button("Create");
                let load = ui.button("Load");


                if save.clicked() {
                    if file_name.name != "" {
                        println!("Creating: {}.{}", file_name.name, FILE_EXT);
                        *editor_step = EditorStep::FileCreated;
                    }
                }

                if load.clicked() {
                    if file_name.name != "" {
                        println!("Loading: {}.{}", file_name.name, FILE_EXT);
                        *editor_step = EditorStep::FileCreated;
                        export_state.set(FileState::Importing);
                    }
                }
            });
        });



    });

}


fn check_settings(
    sprite: Res<Sprite>,
    input_field: Res<InputField>,
) {
    if sprite.height.value != input_field.height || sprite.width.value != input_field.width {
        panic!("WOW, shouldn't happen, de-sync error!");
    }
}