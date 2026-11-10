use crate::ui_lib::highlight_quotes;
use crate::DubiousDubsState;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{self};
use eframe::egui::{Color32, RichText};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

pub fn draw_current_details(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    StripBuilder::new(ui)
        .size(Size::exact(260.0))
        .size(Size::remainder())
        .vertical(|mut vert_strip| {
            vert_strip.strip(|strip_builder| {
                strip_builder
                    .size(Size::exact(350.0))
                    .size(Size::exact(200.0))
                    .size(Size::exact(300.0))
                    .size(Size::exact(300.0))
                    .size(Size::exact(300.0))
                    .horizontal(|mut strip| {
                        // 1st Column
                        strip.cell(|ui| {
                            egui::Frame::canvas(ui.style_mut())
                                .inner_margin(egui::Margin::symmetric(4,0))
                                .corner_radius(5)
                                .show(ui, |ui| {
                                    draw_detail_table_1(ui, state);
                                });
                        });

                        // 2nd Column
                        strip.cell(|ui| {
                            egui::Frame::canvas(ui.style_mut())
                                .inner_margin(egui::Margin::symmetric(4,0))
                                .corner_radius(5)
                                .show(ui, |ui| {
                                    draw_detail_table_2(ui, state);
                                });
                        });

                        strip.cell(|ui| {
                            egui::Frame::canvas(ui.style_mut())
                                .inner_margin(egui::Margin::symmetric(4,0))
                                .corner_radius(5)
                                .show(ui, |ui| {
                                    draw_detail_table_sets(ui, state);
                                });
                        });

                        strip.cell(|ui| {
                            egui::Frame::canvas(ui.style_mut())
                                .inner_margin(egui::Margin::symmetric(4,0))
                                .corner_radius(5)
                                .show(ui, |ui| {
                                    draw_detail_table_path(ui, state);
                                });
                        });
                        strip.cell(|ui| {
                            egui::Frame::canvas(ui.style_mut())
                                .inner_margin(egui::Margin::symmetric(4,0))
                                .corner_radius(5)
                                .show(ui, |ui| {
                                    draw_detail_table_requirements(ui, state);
                                });
                        });
                    });
            });

            vert_strip.cell(|ui| {
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                egui::Frame::canvas(ui.style_mut())
                    .inner_margin(egui::Margin::symmetric(4,0))
                    .corner_radius(5)
                    .show(ui, |ui| {
                        draw_detail_table_line_to_dub(ui, state);
                    });
            });
        });
}

pub fn draw_detail_single_line_label(ui: &mut egui::Ui, text: &str, is_centered: bool) {
    let layout = if is_centered {
        egui::Layout::centered_and_justified(egui::Direction::TopDown)
    } else {
        egui::Layout::left_to_right(egui::Align::Center)
    };
    let styled_text: RichText = RichText::new(text);
    ui.with_layout(layout, |ui| {
        ui.add(egui::Label::new(styled_text).selectable(false))
    });
}


pub fn draw_detail_single_line_value(ui: &mut egui::Ui, text: &str, is_centered: bool) {
    let layout = if is_centered {
        egui::Layout::centered_and_justified(egui::Direction::TopDown)
    } else {
        egui::Layout::left_to_right(egui::Align::Center)
    };
    let styled_text: RichText = RichText::new(text).strong();
    ui.with_layout(layout, |ui| {
        ui.add(egui::Label::new(styled_text).selectable(false).wrap())
    });
}

pub fn draw_detail_multi_line_value(
    ui: &mut egui::Ui,
    text: &str,
    is_centered: bool,
    pattern: &str,
) {
    let layout = if is_centered {
        egui::Layout::centered_and_justified(egui::Direction::TopDown)
    } else {
        egui::Layout::left_to_right(egui::Align::Center)
    };
    let split_text = text.replace(pattern, format!("\n{}", pattern).as_str());
    let styled_text: RichText = RichText::new(split_text.trim()).strong();
    ui.with_layout(layout, |ui| {
        ui.add(egui::Label::new(styled_text).selectable(false).wrap())
    });
}

pub fn draw_dub_line_multi_line_value(
    ui: &mut egui::Ui,
    is_centered: bool,
    state: &mut DubiousDubsState
) {
    let layout = if is_centered {
        egui::Layout::centered_and_justified(egui::Direction::TopDown)
    } else {
        egui::Layout::left_to_right(egui::Align::Center)
    };
    let split_text = state.selected_csv_record.line_to_dub.clone();
    ui.with_layout(layout, |ui| {
        ui.add(egui::Label::new(highlight_quotes(split_text.trim(), state)).selectable(false).wrap())
    });
}

fn draw_detail_checkbox(ui: &mut egui::Ui, value: bool, is_centered: bool) {
    let layout = if is_centered {
        egui::Layout::centered_and_justified(egui::Direction::TopDown)
    } else {
        egui::Layout::left_to_right(egui::Align::Center)
    };
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
    ui.with_layout(layout, |ui| {
        ui.add(egui::Label::new(styled_text).selectable(false))
    });
}

fn draw_detail_table_1(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    TableBuilder::new(ui)
        .striped(true)
        .id_salt("DetailsTable_1")
        .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
        .vscroll(true)
        .max_scroll_height(400.0)
        .min_scrolled_height(200.0)
        .column(
            Column::initial(100.0)
                .at_least(40.0)
                .resizable(false)
                .clip(false),
        )
        .column(
            Column::initial(250.0)
                .at_least(40.0)
                .resizable(false)
                .clip(false),
        )
        .body(|mut body| {
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Dub ID", false);
                });
                row.col(|ui| {
                    draw_detail_single_line_value(ui, &state.selected_csv_record.dub_id, false);
                });
            });
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Story ID", false);
                });
                row.col(|ui| {
                    draw_detail_single_line_value(ui, &state.selected_csv_record.story_id, false);
                });
            });
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Choice ID", false);
                });
                row.col(|ui| {
                    draw_detail_single_line_value(ui, &state.selected_csv_record.choice_id, false);
                });
            });
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Hashcode", false);
                });
                row.col(|ui| {
                    draw_detail_single_line_value(ui, &state.selected_csv_record.hashcode, false);
                });
            });
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Character", false);
                });
                row.col(|ui| {
                    draw_detail_single_line_value(ui, &state.selected_csv_record.character, false);
                });
            });
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Characters", false);
                });
                row.col(|ui| {
                    draw_detail_single_line_value(ui, &state.selected_csv_record.characters, false);
                });
            });
            body.row(50.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Button Text", false);
                });
                row.col(|ui| {
                    draw_detail_single_line_value(
                        ui,
                        &state.selected_csv_record.button_text,
                        false,
                    );
                });
            });
        });
}

fn draw_detail_table_2(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    TableBuilder::new(ui)
        .striped(true)
        .id_salt("DetailsTable_2")
        .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
        .vscroll(true)
        .max_scroll_height(400.0)
        .min_scrolled_height(200.0)
        .column(
            Column::initial(100.0)
                .at_least(40.0)
                .resizable(false)
                .clip(false),
        )
        .column(
            Column::initial(100.0)
                .at_least(40.0)
                .resizable(false)
                .clip(false),
        )
        .body(|mut body| {
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Level", false);
                });
                row.col(|ui| {
                    draw_detail_single_line_value(ui, &state.selected_csv_record.level, false);
                });
            });
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Is Dream", false);
                });
                row.col(|ui| {
                    draw_detail_checkbox(ui, state.selected_csv_record.is_dream.eq("True"), false);
                });
            });
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Is Flirt", false);
                });
                row.col(|ui| {
                    draw_detail_checkbox(ui, state.selected_csv_record.is_flirt.eq("True"), false);
                });
            });
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Is Groundhog", false);
                });
                row.col(|ui| {
                    draw_detail_checkbox(
                        ui,
                        state.selected_csv_record.is_groundhog.eq("True"),
                        false,
                    );
                });
            });
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Has Battle", false);
                });
                row.col(|ui| {
                    draw_detail_checkbox(
                        ui,
                        state.selected_csv_record.has_battle.eq("True"),
                        false,
                    );
                });
            });
        });
}

fn draw_detail_table_path(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    TableBuilder::new(ui)
        .striped(true)
        .id_salt("DetailsTable_Path")
        .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
        .vscroll(true)
        .max_scroll_height(400.0)
        .min_scrolled_height(200.0)
        .column(
            Column::initial(300.0)
                .at_least(40.0)
                .resizable(false)
                .clip(false),
        )
        .body(|mut body| {
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Path", false);
                });
            });

            let line_count = state.selected_csv_record.path.matches(">").count();
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_multi_line_value(ui, &state.selected_csv_record.path, false, " >");
                });
            });
        });
}

fn draw_detail_table_requirements(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    TableBuilder::new(ui)
        .striped(true)
        .id_salt("DetailsTable_Requirements")
        .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
        .vscroll(true)
        .max_scroll_height(400.0)
        .min_scrolled_height(200.0)
        .column(
            Column::initial(300.0)
                .at_least(40.0)
                .resizable(false)
                .clip(false),
        )
        .body(|mut body| {
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Requirements", false);
                });
            });

            let line_count = state
                .selected_csv_record
                .requirements
                .matches("~if")
                .count();
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_multi_line_value(
                        ui,
                        &state.selected_csv_record.requirements,
                        false,
                        "~if",
                    );
                });
            });
        });
}

fn draw_detail_table_sets(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    TableBuilder::new(ui)
        .striped(true)
        .id_salt("DetailsTable_Sets")
        .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
        .vscroll(true)
        .max_scroll_height(400.0)
        .min_scrolled_height(200.0)
        .column(
            Column::initial(300.0)
                .at_least(40.0)
                .resizable(false)
                .clip(false),
        )
        .body(|mut body| {
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Sets", false);
                });
            });

            let line_count = state.selected_csv_record.sets.matches("~set").count();
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_multi_line_value(
                        ui,
                        &state.selected_csv_record.sets,
                        false,
                        "~set",
                    );
                });
            });
        });
}

fn draw_detail_table_line_to_dub(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    TableBuilder::new(ui)
        .striped(true)
        .id_salt("DetailsTable_LineToDub")
        .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
        .vscroll(true)
        .max_scroll_height(400.0)
        .min_scrolled_height(200.0)
        .column(
            Column::initial(1225.0)
                .at_least(40.0)
                .resizable(false)
                .clip(false),
        )
        .body(|mut body| {
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    draw_detail_single_line_label(ui, "Line To Dub", false);
                });
            });
            
            body.row(30.0, |mut row| {
                row.col(|ui| {
                            draw_dub_line_multi_line_value(
                        ui,
                        false,
                        state
                            );
                });
            });
        });
}

// ui.vertical(|ui| {
//     ui.horizontal_centered(|ui| {
//         ui.vertical(|ui| {
//             ui_lib::draw_gridcell_text(ui, "Dub ID: ", &state.selected_csv_record.dub_id);
//             ui_lib::draw_gridcell_text(ui, "Story ID: ", &state.selected_csv_record.story_id);
//             ui_lib::draw_gridcell_text(ui, "Choice ID: ", &state.selected_csv_record.choice_id);
//             ui_lib::draw_gridcell_text(ui, "Hashcode: ", &state.selected_csv_record.hashcode);
//             ui_lib::draw_gridcell_text(ui, "Character: ", &state.selected_csv_record.character);
//             ui_lib::draw_gridcell_text(ui, "Characters: ", &state.selected_csv_record.characters);
//             ui_lib::draw_gridcell_text(ui, "Button Text: ", &state.selected_csv_record.button_text);
//         });

//         ui.vertical(|ui| {
//             ui_lib::draw_gridcell_text(ui, "Level: ", &state.selected_csv_record.level);
//             ui_lib::draw_gridcell_checkbox(
//                 ui,
//                 "Is Dream",
//                 state.selected_csv_record.is_dream.eq("True"),
//             );
//             ui_lib::draw_gridcell_checkbox(
//                 ui,
//                 "Is Flirt",
//                 state.selected_csv_record.is_flirt.eq("True"),
//             );
//             ui_lib::draw_gridcell_checkbox(
//                 ui,
//                 "Is Groundhog",
//                 state.selected_csv_record.is_groundhog.eq("True"),
//             );
//             ui_lib::draw_gridcell_checkbox(
//                 ui,
//                 "Has Battle",
//                 state.selected_csv_record.has_battle.eq("True"),
//             );
//         });
//         ui.vertical(|ui| {
//             ui_lib::draw_gridcell_requirements_text(
//                 ui,
//                 "Requirements: ",
//                 &state.selected_csv_record.requirements,
//             );
//             ui_lib::draw_gridcell_path_text(ui, "Path: ", &state.selected_csv_record.path);
//             ui_lib::draw_gridcell_sets_text(ui, "Sets: ", &state.selected_csv_record.sets);
//         });
//     });
//     ui_lib::draw_gridcell_multiline_text(ui, "Line To Dub: ", &state.selected_csv_record.line_to_dub);
// });
