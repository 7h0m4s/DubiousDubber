use eframe::egui;
use egui_extras::{Column, TableBuilder};
use eframe::egui::scroll_area::ScrollBarVisibility;
use crate::DubiousDubsState;

pub fn generated_ui(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
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

        ui.scope_builder(egui::UiBuilder::new().max_rect(egui::Rect::from_min_size(ui.min_rect().min + egui::vec2(10.0, 10.0), egui::vec2(400.0, 400.0))), |ui| {

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
    });
}