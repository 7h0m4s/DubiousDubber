use crate::audio_lib::{
    start_audio_playback, start_audio_recording, stop_audio_playback, stop_audio_recording,
};
use crate::csv_lib::CsvRecord;
use crate::file_lib::{get_current_attempt_files, get_next_attempt_filename_from_file_list};
use crate::{CurrentMode, DubiousDubsState, csv_lib};
use anyhow::Error;
use eframe::egui::Role::CheckBox;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{self, IdSalt, Layout, Stroke, vec2};
use eframe::egui::{Align, Color32, FontSelection, RichText, Style, Vec2, text};
use egui_extras::{Column, Size, StripBuilder, TableBody, TableBuilder, TableRow};
use std::fmt::Alignment;

mod ui_csv_table;
mod ui_attempt_file_table;


pub fn generated_ui_panels(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    if state.enable_top {
        egui::Panel::top("gen_top").resizable(true).show(ui, |ui| {
            let file_menu_items = vec![
                "Load CSV".to_string(),
                "Temp Audio Folder".to_string(),
                "Raw Audio Folder".to_string(),
            ];
            ui.push_id(("menu_buttons", 13), |ui| {
                ui.menu_button("File", |ui| {
                    for (i, it) in file_menu_items.iter().enumerate() {
                        if ui.button(it).clicked() {
                            state.sel_13 = i;
                            ui.close_kind(egui::UiKind::Menu);
                        }
                    }
                });
            });
        });
    }

    egui::CentralPanel::default().show(ui, |ui| {
        // let canvas = egui::Rect::from_min_size(ui.min_rect().min, egui::vec2(700.0, 600.0));
        // let _ = ui.allocate_painter(canvas.size(), egui::Sense::hover());

        generate_center_ui_elements(ui, state).expect("TODO: panic message");
    });
}

pub fn generate_center_ui_elements(
    ui: &mut egui::Ui,
    state: &mut DubiousDubsState,
) -> Result<(), anyhow::Error> {
    let record_button_text = if state.current_mode == CurrentMode::Recording {
        "Stop Recording"
    } else {
        "Start Recording"
    };
    let record_button_styled_text = RichText::new(record_button_text)
        .size(28.0)
        .color(Color32::WHITE);
    let record_button = egui::Button::new(record_button_styled_text)
        .fill(if state.current_mode == CurrentMode::Recording {
            Color32::DARK_RED
        } else {
            Color32::DARK_GREEN
        })
        .min_size(Vec2::new(125.0, 50.0));

    if ui.add(record_button).clicked() {
        if state.current_mode == CurrentMode::Recording {
            state.current_mode = CurrentMode::Idle;
            stop_audio_recording(&mut state.recorder)?;
        } else {
            state.current_mode = CurrentMode::Recording;
            start_audio_recording(&mut state.recorder, &state.selected_filename.as_str())?;
        }
    }

    // Keep repainting if we are waiting for playback to finish
    if (state.current_mode == CurrentMode::Playback) {
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(50));
    }
    // Check playback status
    if (state.player.empty() || state.player.is_paused())
        && state.current_mode == CurrentMode::Playback
    {
        state.current_mode == CurrentMode::Idle;
    }
    let playback_button_text = if state.current_mode == CurrentMode::Playback {
        "Stop playback"
    } else {
        "Start playback"
    };
    let playback_button_styled_text = RichText::new(playback_button_text)
        .size(28.0)
        .color(Color32::WHITE);
    let playback_button = egui::Button::new(playback_button_styled_text)
        .fill(if state.current_mode == CurrentMode::Playback {
            Color32::DARK_RED
        } else {
            Color32::DARK_GREEN
        })
        .min_size(Vec2::new(125.0, 50.0));

    if ui.add(playback_button).clicked() {
        if state.current_mode == CurrentMode::Playback {
            state.current_mode = CurrentMode::Idle;
            stop_audio_playback(&mut state.player)?;
        } else {
            state.current_mode = CurrentMode::Playback;
            start_audio_playback(&mut state.player, &state.selected_filename.as_str())?;
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(50));
        }
    }

    let mut selected_filename_label_response = ui.add(egui::Label::new(format!(
        "Filename to generate: {}",
        &state.selected_filename
    )));
    selected_filename_label_response.set_intrinsic_size(Vec2::new(250.0, 100.0));

    egui::Frame::canvas(ui.style_mut())
        .inner_margin(4)
        .corner_radius(3)
        .show(ui, |ui| {
            ui_csv_table::draw_csv_table(ui, state).expect("CsvTable");
        });

    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);

    // egui::Frame::canvas(ui.style_mut())
    //     .inner_margin(4)
    //     .corner_radius(3)
    //     .show(ui, |ui| {
    //         draw_attempt_file_table(ui, state).expect("AttemptTable");
    //     });

    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);

    
   

    StripBuilder::new(ui)
    .size(Size::exact(1000.0))
    .size(Size::remainder())
    .horizontal(|mut strip|{

        strip.cell(|ui|{
            draw_current_details(ui, state);
        });
        strip.cell(|ui|{
            ui_attempt_file_table::draw_attempt_file_table(ui, state).expect("AttemptTable");
        });
    });

    

    Ok(())
}

fn draw_current_details(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    ui.vertical(|ui| {
        ui.horizontal_centered(|ui| {
            ui.vertical(|ui| {
                draw_gridcell_text(ui, "Dub ID: ", &state.selected_csv_record.dub_id);
                draw_gridcell_text(ui, "Story ID: ", &state.selected_csv_record.story_id);
                draw_gridcell_text(ui, "Choice ID: ", &state.selected_csv_record.choice_id);
                draw_gridcell_text(ui, "Hashcode: ", &state.selected_csv_record.hashcode);
                draw_gridcell_text(ui, "Character: ", &state.selected_csv_record.character);
                draw_gridcell_text(ui, "Characters: ", &state.selected_csv_record.characters);
                draw_gridcell_text(ui, "Button Text: ", &state.selected_csv_record.button_text);
            });

            ui.vertical(|ui| {
                
                draw_gridcell_text(ui, "Level: ", &state.selected_csv_record.level);
                draw_gridcell_checkbox(
                    ui,
                    "Is Dream",
                    state.selected_csv_record.is_dream.eq("True"),
                );
                draw_gridcell_checkbox(
                    ui,
                    "Is Flirt",
                    state.selected_csv_record.is_flirt.eq("True"),
                );
                draw_gridcell_checkbox(
                    ui,
                    "Is Groundhog",
                    state.selected_csv_record.is_groundhog.eq("True"),
                );
                draw_gridcell_checkbox(
                    ui,
                    "Has Battle",
                    state.selected_csv_record.has_battle.eq("True"),
                );
            });
            ui.vertical(|ui| {
                draw_gridcell_requirements_text(
                    ui,
                    "Requirements: ",
                    &state.selected_csv_record.requirements,
                );
                draw_gridcell_path_text(ui, "Path: ", &state.selected_csv_record.path);
                draw_gridcell_sets_text(ui, "Sets: ", &state.selected_csv_record.sets);
                
            });
        });
        draw_gridcell_multiline_text(ui, "Line To Dub: ", &state.selected_csv_record.line_to_dub);
    });
}

fn draw_gridcell_text(ui: &mut egui::Ui, label: &str, text: &str) {
    ui.horizontal(|ui| {
        egui::Frame::canvas(ui.style_mut())
            .inner_margin(4)
            .corner_radius(5)
            .show(ui, |ui| {
                ui.label(label);
                let styled_text: RichText = RichText::new(text).strong();
                ui.add(egui::Label::new(styled_text).wrap_mode(egui::TextWrapMode::Wrap))
            });
    });
}
fn draw_gridcell_multiline_text(ui: &mut egui::Ui, label: &str, text: &str) {
    let mut modified_text = text.to_string();
    ui.horizontal(|ui| {
        egui::Frame::canvas(ui.style_mut())
            .inner_margin(4)
            .corner_radius(5)
            .show(ui, |ui| {
                ui.label(label);
                let styled_text: RichText = RichText::new(text).strong().size(20.0);
                ui.add(egui::Label::new(styled_text).wrap_mode(egui::TextWrapMode::Wrap))
            });
    });
}

fn draw_gridcell_sets_text(ui: &mut egui::Ui, label: &str, text: &str) {
    let mut modified_text = text.to_string();
    modified_text = modified_text.replace(", ~set ", "\n").replace("~set ", "");
    draw_gridcell_text(ui, label, &modified_text.to_string());
}

fn draw_gridcell_requirements_text(ui: &mut egui::Ui, label: &str, text: &str) {
    let mut modified_text = text.to_string();
    modified_text = modified_text.replace(", ~if ", "\n~if");
    draw_gridcell_text(ui, label, &modified_text.to_string());
}

fn draw_gridcell_path_text(ui: &mut egui::Ui, label: &str, text: &str) {
    let mut modified_text = text.to_string();
    modified_text = modified_text.replace(" >", "\n>").replacen("\n", "", 1);
    draw_gridcell_text(ui, label, &modified_text.to_string());
}

fn draw_gridcell_checkbox(ui: &mut egui::Ui, label: &str, value: bool) {
    ui.horizontal(|ui| {
        egui::Frame::canvas(ui.style_mut())
            .inner_margin(4)
            .corner_radius(5)
            .show(ui, |ui| {
                let mut checkbox_mut_value = value;
                let label_response = ui.label(label);
                let text: &str;

                if value {
                    text = "True";
                } else {
                    text = "False";
                }
                let mut styled_text: RichText = RichText::new(text).strong();
                if value {
                    styled_text = styled_text.color(Color32::GREEN);
                } else {
                    styled_text = styled_text.color(Color32::RED);
                }
                ui.add(egui::Label::new(styled_text).wrap_mode(egui::TextWrapMode::Wrap))
            });
    });
}



fn format_table_cell(ui: &mut egui::Ui, is_selected: bool, is_hovered: bool, text: &String) {
    let styled_text: RichText;
    if is_selected {
        //ui.painter().rect_filled(ui.max_rect(), 0.2, Color32::DARK_GREEN);
        styled_text = RichText::new(text).color(Color32::WHITE);
    } else if is_hovered {
        //ui.painter().rect_filled(ui.max_rect(), 0.2, Color32::LIGHT_BLUE);
        styled_text = RichText::new(text).strong();
    } else {
        styled_text = RichText::new(text).color(Color32::LIGHT_GRAY);
    }

    ui.centered_and_justified(|ui| ui.add(egui::Label::new(styled_text).selectable(false)));
}


