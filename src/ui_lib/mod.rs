use crate::audio_lib::{
    start_audio_playback, start_audio_recording, stop_audio_playback, stop_audio_recording,
};
use crate::csv_lib::{parse_csv_records, CsvRecord, initialize_csv_state};
use crate::file_lib::get_current_attempt_files;
use crate::{CurrentMode, DubiousDubsState, csv_lib};
use anyhow::{anyhow, Error};
use eframe::egui::Role::CheckBox;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{self, IdSalt, Layout, Stroke, vec2};
use eframe::egui::{Align, Color32, FontSelection, RichText, Style, Vec2, text};
use egui_extras::{Column, Size, StripBuilder, TableBody, TableBuilder, TableRow};
use std::fmt::Alignment;
use rfd::FileDialog;
use crate::ui_lib::ui_menu::draw_menu;

mod ui_attempt_file_table;
mod ui_bottom_buttons;
mod ui_csv_table;
mod ui_current_details;
mod ui_upper_controls;
mod ui_menu;



pub fn generated_ui_panels(ui: &mut egui::Ui, state: &mut DubiousDubsState) {

    //ui.set_visuals(egui::Visuals::light());
    if state.enable_top {
        egui::Panel::top("gen_top").resizable(true).show(ui, |ui| {
            draw_menu(ui, state)
        });
    }

    // show bottom first so it has first dibs on space
    egui::Panel::bottom("gen_bottom")
    .resizable(false)
    .default_size(10.0)
    .show(ui, |ui| {
        egui::Frame::default()
        .fill(Color32::TRANSPARENT)
        .inner_margin(10.0)
        .show(ui, |ui|{
            ui_bottom_buttons::draw_bottom_buttons(ui, state)
            .expect("Failed to render bottom buttons.");
        });
        
    });

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
    StripBuilder::new(ui)
        .size(Size::exact(state.ui_csv_table_height))
        .size(Size::remainder())
        .vertical(|mut strip| {
            strip.cell(|ui| {
                ui.set_min_height(state.ui_csv_table_height);
                //ui.add(egui::DragValue::new(&mut state.ui_csv_table_height));
                
                ui_csv_table::draw_csv_table(ui, state).expect("Failed to render CSV table.");
                ui.add_space(10.0);
            });
            strip.strip(|strip_builder| {
                strip_builder
                    .size(Size::exact(400.0))
                    .size(Size::remainder())
                    .clip(true)
                    .horizontal(|mut horizontal_strip| {
                        horizontal_strip.cell(|ui| {
                            ui.set_clip_rect(ui.max_rect());
                            ui_attempt_file_table::draw_attempt_file_table(ui, state)
                                .expect("AttemptTable");
                        });

                        horizontal_strip.cell(|ui| {
                            ui_current_details::draw_current_details(ui, state);
                        });
                    });
            });
        });

    // StripBuilder::new(ui)
    //     .size(Size::remainder())
    //     .size(Size::exact(400.0))
    //     .clip(true)
    //     .horizontal(|mut strip| {
    //         strip.cell(|ui| {
    //             draw_current_details(ui, state);
    //         });
    //         strip.cell(|ui| {
    //             ui_attempt_file_table::draw_attempt_file_table(ui, state).expect("AttemptTable");
    //         });
    //     });

    // ui.add_space(5.0);
    // ui.separator();
    // ui.add_space(5.0);

    // ui_bottom_buttons::draw_bottom_buttons(ui, state)?;

    Ok(())
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

fn format_table_cell(
    ui: &mut egui::Ui,
    is_selected: bool,
    is_hovered: bool,
    is_committed: bool,
    is_centered: bool,
    text: &String,
) {
    let mut styled_text: RichText;
    if is_selected {
        //ui.painter().rect_filled(ui.max_rect(), 0.2, Color32::DARK_GREEN);
        styled_text = RichText::new(text).color(Color32::WHITE);
    } else if is_hovered {
        //ui.painter().rect_filled(ui.max_rect(), 0.2, Color32::LIGHT_BLUE);
        styled_text = RichText::new(text).strong();
    } else {
        styled_text = RichText::new(text).color(Color32::LIGHT_GRAY);
    }

    if is_committed {
        styled_text = styled_text.italics().color(Color32::LIGHT_GREEN);
    }

    let layout = if is_centered {
        egui::Layout::centered_and_justified(egui::Direction::TopDown)
    } else {
        egui::Layout::left_to_right(egui::Align::Center)
    };

    ui.with_layout(layout, |ui| {
        ui.add(egui::Label::new(styled_text).selectable(false))
    });
}
