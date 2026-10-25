use std::fmt::Alignment;
use anyhow::Error;
use eframe::egui;
use eframe::egui::{Color32, Vec2, RichText, Align, FontSelection, Style, text};
use egui_extras::{Column, Size, TableBuilder};
use eframe::egui::scroll_area::ScrollBarVisibility;
use crate::{CurrentMode, DubiousDubsState};
use crate::audio_recorder_worker::{start_audio_recording, stop_audio_recording};

pub fn generated_ui_panels(ui: &mut egui::Ui, state: &mut DubiousDubsState) {



    if state.enable_top {
        egui::Panel::top("gen_top")
            .resizable(true)
            .show(ui, |ui| {
                let file_menu_items = vec!["Load CSV".to_string(), "Temp Audio Folder".to_string(), "Raw Audio Folder".to_string()];
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

        generate_center_ui_elements(ui, state);

    });
}

pub fn generate_center_ui_elements(ui: &mut egui::Ui, state: &mut DubiousDubsState) -> Result<(),anyhow::Error> {
    let record_button_text = if state.current_mode == CurrentMode::Recording {"Stop Recording"} else {"Start Recording"};
    let record_button_styled_text =
        RichText::new(record_button_text)
            .size(28.0).color(Color32::WHITE);
    let record_button = egui::Button::new(record_button_styled_text)
        .fill(if state.current_mode == CurrentMode::Recording {Color32::DARK_RED} else {Color32::DARK_GREEN})
        .min_size(Vec2::new(250.0, 100.0));

    if ui.add(record_button)
        .clicked() {
        if state.current_mode == CurrentMode::Recording {

            state.current_mode = CurrentMode::Idle;
            stop_audio_recording(&mut state.recorder)?;
        } else{
            state.current_mode = CurrentMode::Recording;
            start_audio_recording(&mut state.recorder,&state.selected_filename.as_str())?;
        }
    }

    let mut editable_textbox_response = ui.add(egui::TextEdit::singleline(&mut state.selected_filename));
    editable_textbox_response.set_intrinsic_size(Vec2::new(250.0, 100.0));




    ui.scope_builder(egui::UiBuilder::new().max_rect(egui::Rect::from_min_size(ui.min_rect().min + egui::vec2(10.0, 300.0), egui::vec2(400.0, 400.0))), |ui| {

        egui::Frame::canvas(ui.style()).inner_margin(4).corner_radius(3).show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                .vscroll(true)
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("First column");
                    });
                    header.col(|ui| {
                        ui.heading("Second column");
                    });
                })
                .body(|mut body| {
                    for i in 0..7 {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label("Hello");
                            });
                            row.col(|ui| {
                                ui.button("world!");
                            });
                        });
                    }

                });
        });
    });
    Ok(())
}