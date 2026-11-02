use audio_recorder_rs::Recorder;
use std::env::var;
use std::fs::File;
use std::io::{BufReader, sink};
use std::path::PathBuf;
use std::str::FromStr;

use rodio::cpal::traits::HostTrait;
use rodio::source::SineWave;
use rodio::{Decoder, DeviceTrait, Sink, Source, cpal};
use std::thread;
use std::time::Duration;

use crate::{CurrentMode, DubiousDubsState};

pub fn start_audio_recording(state: &mut DubiousDubsState) -> Result<(), anyhow::Error> {
    state.current_mode = CurrentMode::Recording;
    let device = cpal::default_host().default_input_device().unwrap();
    let config = device.default_input_config().unwrap();
    let channels = config.channels();
    let sample_rate = config.sample_rate();
    let bits_per_sample = config.sample_format().sample_size() * 8;
    let sample_format = config.sample_format();

    let spec = hound::WavSpec {
        channels: channels,
        sample_rate: sample_rate.0,
        bits_per_sample: bits_per_sample as u16,
        sample_format: if sample_format.is_float() {
            hound::SampleFormat::Float
        } else {
            hound::SampleFormat::Int
        },
    };
    let mut writer = hound::WavWriter::create(&state.next_attempt_path, spec)?;

    let receiver = state
        .recorder
        .start(true)
        .expect("Failed to start audio recording");

    let thread_handle = thread::spawn(move || {
        while let Ok(d) = receiver.recv() {
            for sample in d {
                writer.write_sample(sample).ok();
            }
        }
    });

    //sleep(Duration::from_secs(5));
    //Ok(recorder)
    Ok(())
}

pub fn stop_audio_recording(state: &mut DubiousDubsState) -> Result<(), anyhow::Error> {
    state.current_mode = CurrentMode::Idle;
    state.recorder.stop();
    Ok(())
}

pub fn start_audio_playback(state: &mut DubiousDubsState, attempts: usize, is_attempt_file:bool) {
    state.current_mode = CurrentMode::Playback;
    state.player.stop();
    state.player.clear();

    let mut audio_file_path = PathBuf::from_str(&state.attempt_directory).unwrap();
    audio_file_path.push(&state.selected_attempt_filename);


    let audio_file_path :PathBuf;
    if is_attempt_file{
        audio_file_path= [&state.attempt_directory, &state.selected_attempt_filename].iter().collect();
    } else {
        audio_file_path= [&state.commit_directory, &format!("{}.wav", state.selected_dub_id)].iter().collect();
    }

    let mut attempts_left = attempts;
    while (attempts_left > 0) {
        let file = match File::open(&audio_file_path) {
            Ok(f) => f,
            Err(err) => {
                //eprintln!("failed to open wav file: {}", err);
                attempts_left -= 1;
                thread::sleep(Duration::from_millis(50));
                continue;
            }
        };
        let decoder = match Decoder::new(file) {
            Ok(f) => f,
            Err(err) => {
                attempts_left -= 1;
                thread::sleep(Duration::from_millis(50));
                continue;
            }
        };
        state.player.append(decoder);
        state.player.play();
        return;
    }
    eprintln!(
        "failed to open wav file: {}",
        audio_file_path.to_string_lossy()
    );
}

pub fn stop_audio_playback(state: &mut DubiousDubsState) -> Result<(), anyhow::Error> {
    state.current_mode = CurrentMode::Idle;
    state.player.stop();
    Ok(())
}
