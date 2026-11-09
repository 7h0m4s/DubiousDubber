use crate::ui_lib::ui_menu::draw_menu;
use crate::ui_lib::ui_upper_controls::draw_upper_controls;
use crate::{CurrentMode, DubiousDubsState, csv_lib};
use eframe::egui::{self, IdSalt, Layout, Stroke, vec2};
use eframe::egui::{Align, Color32, FontSelection, RichText, Style, Vec2, text};
use eframe::epaint::text::{LayoutJob, TextFormat};
use eframe::epaint::{FontFamily, FontId};
use egui_extras::{Column, Size, StripBuilder, TableBody, TableBuilder, TableRow};
use std::collections::HashMap;
use std::iter::Filter;

mod ui_attempt_file_table;
mod ui_bottom_buttons;
mod ui_csv_table;
mod ui_current_details;
mod ui_menu;
mod ui_upper_controls;


pub fn generated_ui_panels(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    ui.spacing_mut().item_spacing.y = -0.0;
    if state.enable_top {
        egui::Panel::top("gen_top")
            .resizable(false)
            .show(ui, |ui| draw_menu(ui, state));
    }

    // show bottom first so it has first dibs on space
    egui::Panel::bottom("gen_bottom")
        .resizable(false)
        .default_size(10.0)
        .show(ui, |ui| {
            egui::Frame::default()
                .fill(Color32::TRANSPARENT)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui_bottom_buttons::draw_bottom_buttons(ui, state)
                        .expect("Failed to render bottom buttons.");
                });
        });

    egui::CentralPanel::default().show(ui, |ui| {
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

                draw_upper_controls(ui, state);
                ui_csv_table::draw_csv_table(ui, state);
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
                        });

                        horizontal_strip.cell(|ui| {
                            ui_current_details::draw_current_details(ui, state);
                        });
                    });
            });
        });

    Ok(())
}

pub fn set_display_visuals(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    if state.is_light_mode {
        ui.set_visuals(egui::Visuals::light());
        ui.ctx().global_style_mut(|s| {
            s.visuals.panel_fill = Color32::from_gray(210);
            s.visuals.extreme_bg_color = Color32::from_gray(230);
            s.visuals.faint_bg_color = Color32::from_gray(220);
        });
    } else {
        ui.set_visuals(egui::Visuals::dark());
        ui.ctx().global_style_mut(|s| {
            s.visuals.panel_fill = Color32::from_gray(45);
            s.visuals.extreme_bg_color = Color32::from_gray(25);
            s.visuals.faint_bg_color = Color32::from_gray(35);
        });
    }
}

fn format_table_cell(
    ui: &mut egui::Ui,
    is_selected: bool,
    is_hovered: bool,
    is_committed: bool,
    is_centered: bool,
    text: &String,
    is_light_mode: bool,
) {
    let mut styled_text: RichText;
    if is_selected {
        //ui.painter().rect_filled(ui.max_rect(), 0.2, Color32::DARK_GREEN);
        styled_text = RichText::new(text)
            .color(if is_light_mode {
                Color32::BLACK
            } else {
                Color32::WHITE
            })
            .strong();
    } else if is_hovered {
        //ui.painter().rect_filled(ui.max_rect(), 0.2, Color32::LIGHT_BLUE);
        styled_text = RichText::new(text).strong();
    } else {
        styled_text = RichText::new(text).color(if is_light_mode {
            Color32::BLACK
        } else {
            Color32::LIGHT_GRAY
        });
    }

    if is_committed {
        styled_text = styled_text.italics().color(if is_light_mode {
            Color32::DARK_GREEN
        } else {
            Color32::LIGHT_GREEN
        });
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


pub fn highlight_quotes(in_string: &str, state: &mut DubiousDubsState) -> LayoutJob {
    let regular_color = if state.is_light_mode {
        Color32::BLACK
    } else {
        Color32::WHITE
    };

    let highlight_color = if state.is_light_mode {
        Color32::from_rgb(0, 90, 200)
    } else {
        Color32::from_rgb(100, 180, 255)
    };

    let mut job = LayoutJob::default();
    let font = FontId::new(26.0, FontFamily::Name("bold".into()));
    let regular_format = TextFormat {
        font_id: font.to_owned(),
        color: regular_color,
        ..Default::default()
    };
    let quote_format = TextFormat {
        font_id: font.to_owned(),
        color: highlight_color,
        ..Default::default()
    };

    let mut in_quotes = false;
    let mut start = 0;

    for (i, c) in in_string.char_indices() {
        if c == '"' {
            if !in_quotes {
                // flush normal text before opening quote
                if start < i {
                    job.append(&in_string[start..i], 0.0, regular_format.clone());
                }
                start = i;
                in_quotes = true;
            } else {
                let end = i + c.len_utf8();
                job.append(&in_string[start..end], 0.0, quote_format.clone());
                start = end;
                in_quotes = false;
            }
        }
    }

    // Flush whatever remains (text after the last quote, or all text if there were no quotes)
    if start < in_string.len() {
        let fmt = if in_quotes {
            quote_format
        } else {
            regular_format
        };
        job.append(&in_string[start..], 0.0, fmt);
    }

    job
}
