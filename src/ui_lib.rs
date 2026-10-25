use std::fmt::Alignment;
use anyhow::Error;
use eframe::egui;
use eframe::egui::{Color32, Vec2, RichText, Align, FontSelection, Style, text};
use egui_extras::{Column, Size, TableBody, TableBuilder, TableRow};
use eframe::egui::scroll_area::ScrollBarVisibility;
use crate::{csv_lib, CurrentMode, DubiousDubsState};
use crate::audio_lib::{start_audio_playback, start_audio_recording, stop_audio_playback, stop_audio_recording};
use crate::csv_lib::CsvRecord;

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

        generate_center_ui_elements(ui, state).expect("TODO: panic message");

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



    // Keep repainting if we are waiting for playback to finish
    if(state.current_mode == CurrentMode::Playback) {ui.ctx().request_repaint_after(std::time::Duration::from_millis(50));}
    // Check playback status
    if (state.player.empty() || state.player.is_paused()) && state.current_mode == CurrentMode::Playback{
        state.current_mode == CurrentMode::Idle;
    }
    let playback_button_text = if state.current_mode == CurrentMode::Playback {"Stop playback"} else {"Start playback"};
    let playback_button_styled_text =
        RichText::new(playback_button_text)
            .size(28.0).color(Color32::WHITE);
    let playback_button = egui::Button::new(playback_button_styled_text)
        .fill(if state.current_mode == CurrentMode::Playback {Color32::DARK_RED} else {Color32::DARK_GREEN})
        .min_size(Vec2::new(250.0, 100.0));

    if ui.add(playback_button)
        .clicked() {
        if state.current_mode == CurrentMode::Playback {

            state.current_mode = CurrentMode::Idle;
            stop_audio_playback(&mut state.player)?;
        } else{
            state.current_mode = CurrentMode::Playback;
            start_audio_playback(&mut state.player,&state.selected_filename.as_str())?;
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(50));
        }
    }

    let mut editable_textbox_response = ui.add(egui::TextEdit::singleline(&mut state.selected_filename));
    editable_textbox_response.set_intrinsic_size(Vec2::new(250.0, 100.0));




    ui.scope_builder(egui::UiBuilder::new().max_rect(egui::Rect::from_min_size(ui.min_rect().min + egui::vec2(10.0, 300.0), egui::vec2(400.0, 400.0))), |ui| {

        egui::Frame::canvas(ui.style()).inner_margin(4).corner_radius(3).show(ui, |ui| {
            generate_csv_table(ui, state).expect("CsvTable");
        });
    });
    Ok(())
}

fn generate_csv_table(ui: &mut egui::Ui, state: &mut DubiousDubsState) -> Result<(),Error> {
     TableBuilder::new(ui)
        .striped(true)
        .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
        .vscroll(true)
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .header(20.0, |mut header| {
            header.col(|ui| { ui.heading("Story ID"); });
            header.col(|ui| { ui.heading("Choice ID"); });
            header.col(|ui| { ui.heading("Hashcode"); });
            header.col(|ui| { ui.heading("Filename"); });
            header.col(|ui| { ui.heading("Character"); });
            header.col(|ui| { ui.heading("Characters"); });
            header.col(|ui| { ui.heading("Button Text"); });
            header.col(|ui| { ui.heading("Sets"); });
            header.col(|ui| { ui.heading("Line To Dub"); });
            header.col(|ui| { ui.heading("Requirements"); });
            header.col(|ui| { ui.heading("Path"); });
            header.col(|ui| { ui.heading("Level"); });
            header.col(|ui| { ui.heading("Is Dream"); });
            header.col(|ui| { ui.heading("Is Flirt"); });
            header.col(|ui| { ui.heading("Is Groundhog"); });
            header.col(|ui| { ui.heading("Has Battle"); });

        })
        .body(|mut body| {

            //generate_csv_rows(&mut body, state).expect("TODO: panic message");
            let records = &state.csv_records;
            body.rows(30.0, records.len(), |mut row| {
                let rec = &records[row.index()];
                row.col(|ui| { ui.label(&rec.story_id); });
                row.col(|ui| { ui.label(&rec.choice_id); });
                row.col(|ui| { ui.label(&rec.hashcode); });
                row.col(|ui| { ui.label(&rec.filename); });
                row.col(|ui| { ui.label(&rec.character); });
                row.col(|ui| { ui.label(&rec.characters); });
                row.col(|ui| { ui.label(&rec.button_text); });
                row.col(|ui| { ui.label(&rec.sets); });
                row.col(|ui| { ui.label(&rec.line_to_dub); });
                row.col(|ui| { ui.label(&rec.requirements); });
                row.col(|ui| { ui.label(&rec.path); });
                row.col(|ui| { ui.label(&rec.level); });
                row.col(|ui| { ui.label(&rec.is_dream); });
                row.col(|ui| { ui.label(&rec.is_flirt); });
                row.col(|ui| { ui.label(&rec.is_groundhog); });
                row.col(|ui| { ui.label(&rec.has_battle); });
            });
        });
    Ok(())
}



// row.col(|ui| {
// ui.label("Hello");
// });