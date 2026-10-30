use std::fmt::Alignment;
use anyhow::Error;
use eframe::egui::{self, IdSalt, Stroke};
use eframe::egui::{Color32, Vec2, RichText, Align, FontSelection, Style, text};
use egui_extras::{Column, Size, TableBody, TableBuilder, TableRow};
use eframe::egui::scroll_area::ScrollBarVisibility;
use crate::file_lib::{get_current_attempt_files, get_next_attempt_filename_from_file_list};
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
        .min_size(Vec2::new(125.0, 50.0));

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
        .min_size(Vec2::new(125.0, 50.0));

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

    let mut selected_filename_label_response = ui.add(egui::Label::new(format!("Filename to generate: {}",&state.selected_filename)));
    selected_filename_label_response.set_intrinsic_size(Vec2::new(250.0, 100.0));

    egui::Frame::canvas(ui.style_mut()).inner_margin(4).corner_radius(3).show(ui, |ui| {
        draw_csv_table(ui, state).expect("CsvTable");
    });
    
    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);

    egui::Frame::canvas(ui.style_mut()).inner_margin(4).corner_radius(3).show(ui, |ui| {
    draw_attempt_file_table(ui, state).expect("AttemptTable");
    });

    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);

    draw_current_details(ui, state);

    Ok(())
}


fn draw_current_details(ui: &mut egui::Ui, state: &mut DubiousDubsState){
    let cell = egui::vec2(300.0,ui.spacing().interact_size.y);
    ui.horizontal_wrapped(|ui| {
        ui.allocate_ui_with_layout(cell, egui::Layout::left_to_right(egui::Align::Center), |ui|{

            ui.horizontal(|ui|{
                draw_gridcell_text(ui, "Character", &state.selected_csv_record.character);

                draw_gridcell_checkbox(ui, "Is Dream", state.selected_csv_record.is_dream.eq("False"));
            })

        });

    });

}

fn draw_gridcell_text(ui: &mut egui::Ui, label: &str, text: &str){
    ui.horizontal(|ui|{

        ui.label(label);
        let styled_text:RichText = RichText::new(text).strong();
        ui.label(styled_text);
    });
}

fn draw_gridcell_checkbox(ui: &mut egui::Ui, label: &str, value: bool){
    ui.horizontal(|ui|{

        let mut checkbox_mut_value = value;
        ui.label(label);
        let checkbox_response = ui.checkbox(&mut checkbox_mut_value, label);
        checkbox_response.interact(egui::Sense::empty())
    });
}



fn draw_csv_table(ui: &mut egui::Ui, state: &mut DubiousDubsState) -> Result<(),Error> {

     TableBuilder::new(ui)
        .striped(true)
        .sense(egui::Sense::click())
        .id_salt("csvtable")
        .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
        .vscroll(true)
        .max_scroll_height(200.0)
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
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .header(20.0, |mut header| {
            header.col(|ui| { ui.heading("Dub ID"); });
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
                let mut is_selected = row.index().eq(&state.selected_dub_index);
                let mut is_hovered = row.index().eq(&state.hovered_dub_index);
                row.set_selected(is_selected);
                row.set_overline(true);

                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.dub_id); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.story_id); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.choice_id); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.hashcode); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.filename); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.character); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.characters); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.button_text); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.sets); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.line_to_dub); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.requirements); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.path); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.level); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.is_dream); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.is_flirt); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.is_groundhog); });
                row.col(|ui| { format_table_cell(ui, is_selected,is_hovered, &rec.has_battle); });
                
                let is_clicked: bool = row.response().clicked();
                is_hovered = row.response().hovered();
                
                if is_clicked{
                    state.selected_dub_id = rec.dub_id.to_string();
                    state.selected_dub_index = row.index();
                    state.selected_filename = rec.filename.to_string();
                    state.selected_csv_record = rec.clone();
                    state.selected_attempt_filename.clear();
                }

                if is_hovered{
                    state.hovered_dub_index = row.index();
                }

            });
        });
    Ok(())
}


fn format_table_cell(ui:&mut egui::Ui, is_selected: bool, is_hovered:bool, text: &String) {

    let styled_text:RichText;
    if is_selected {
        //ui.painter().rect_filled(ui.max_rect(), 0.2, Color32::DARK_GREEN);
        styled_text =
        RichText::new(text).color(Color32::WHITE);
    }
    else if is_hovered{
        //ui.painter().rect_filled(ui.max_rect(), 0.2, Color32::LIGHT_BLUE);
        styled_text =
        RichText::new(text).strong();
    }
    else {
        styled_text =
        RichText::new(text).color(Color32::LIGHT_GRAY);
    }
    
    ui.centered_and_justified(|ui|{
        ui.add(egui::Label::new(styled_text).selectable(false))
    });

}

fn draw_attempt_file_table(ui: &mut egui::Ui, state: &mut DubiousDubsState) -> Result<(),Error> {

    let current_files = get_current_attempt_files(state).unwrap();
    TableBuilder::new(ui)
        .striped(true)
        .sense(egui::Sense::click())
        .id_salt("attempttable")
        .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
        .vscroll(true)
        .max_scroll_height(200.0)
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .column(Column::initial(80.0).at_least(40.0).resizable(true).clip(true))
        .header(20.0, |mut header| {
            header.col(|ui| { ui.heading("Attempt ID"); });
            header.col(|ui| { ui.heading("Filename"); });

        })
        .body(|mut body| {
            //generate_csv_rows(&mut body, state).expect("TODO: panic message");
            
            body.rows(30.0, current_files.len(), |mut row| {
                let rec = &current_files[row.index()];
                let is_selected = rec.eq(&state.selected_attempt_filename);
                row.set_selected(is_selected);
                row.set_overline(true);
                let row_id = row.index();
                row.col(|ui| { format_table_cell(ui, false,false, &row_id.to_string()); });
                row.col(|ui| {format_table_cell(ui, false,false, &rec.to_string()); });

                let is_clicked: bool = row.response().clicked();
                let is_hovered = row.response().hovered();
                
                if is_clicked{
                    state.selected_attempt_filename = rec.to_string();
                }

                if is_hovered{
                    state.hovered_attempt_filename = rec.to_string()
                }
            });
        });
        let next_filename = get_next_attempt_filename_from_file_list(&current_files, state).unwrap();

        ui.add_space(15.0);

        ui.label(next_filename);
    Ok(())
}



// row.col(|ui| {
// ui.label("Hello");
// });