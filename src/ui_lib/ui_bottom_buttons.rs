use crate::audio_lib::{
    start_audio_playback, start_audio_recording, stop_audio_playback, stop_audio_recording,
};
use crate::csv_lib::CsvRecord;
use crate::file_lib::{get_current_attempt_files, get_next_attempt_filename_from_file_list};
use crate::{CurrentMode, DubiousDubsState, csv_lib};
use anyhow::{Error, Ok};
use eframe::egui::Role::CheckBox;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{self, IdSalt, Layout, Stroke, vec2};
use eframe::egui::{Align, Color32, FontSelection, RichText, Style, Vec2, text};
use egui_extras::{Column, Size, StripBuilder, TableBody, TableBuilder, TableRow};
use std::fmt::Alignment;

pub fn draw_bottom_buttons(
    ui: &mut egui::Ui,
    state: &mut DubiousDubsState,
) -> Result<(), anyhow::Error> {


    ui.horizontal(|ui| {
        
        let record_button_text = if state.current_mode == CurrentMode::Recording {
            "Stop Recording"
        } else {
            "Start Recording"
        };
        let record_button_styled_text = RichText::new(record_button_text)
            .size(24.0)
            .color(Color32::WHITE);
        let record_button = egui::Button::new(record_button_styled_text)
            .fill(if state.current_mode == CurrentMode::Recording {
                Color32::DARK_RED
            } else {
                Color32::DARK_GREEN
            })
            .min_size(Vec2::new(100.0, 50.0));

        if ui.add(record_button).clicked() {
            if state.current_mode == CurrentMode::Recording {
                state.current_mode = CurrentMode::Idle;
                stop_audio_recording(&mut state.recorder);
            } else {
                state.current_mode = CurrentMode::Recording;
                start_audio_recording(&mut state.recorder, &state.selected_filename.as_str());
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
            .min_size(Vec2::new(100.0, 50.0));

        if ui.add(playback_button).clicked() {
            if state.current_mode == CurrentMode::Playback {
                state.current_mode = CurrentMode::Idle;
                stop_audio_playback(&mut state.player);
            } else {
                state.current_mode = CurrentMode::Playback;
                start_audio_playback(&mut state.player, &state.selected_filename.as_str());
                ui.ctx()
                    .request_repaint_after(std::time::Duration::from_millis(50));
            }
        }
    });

    Ok(())
}
