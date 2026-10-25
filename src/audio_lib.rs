use audio_recorder_rs::Recorder;
use std::env::var;
use std::fs::File;
use std::io::{BufReader, sink};

use rodio::cpal::traits::HostTrait;
use rodio::source::SineWave;
use rodio::{Decoder, DeviceTrait, Sink, Source, cpal};
use std::thread;
use std::time::Duration;

pub fn start_audio_recording(
    mut recorder: &mut Recorder,
    filename: &str,
) -> Result<(), anyhow::Error> {
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
    let mut writer = hound::WavWriter::create(filename, spec)?;

    let receiver = recorder
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

pub fn stop_audio_recording(mut recorder: &mut Recorder) -> Result<(), anyhow::Error> {
    recorder.stop();
    Ok(())
}

pub fn start_audio_playback(player: &mut Sink, filename: &str) -> Result<(), anyhow::Error> {
    player.stop();
    player.clear();
    let file = File::open(filename).unwrap();
    let decoder = Decoder::new(file).unwrap();
    player.append(decoder);
    player.play();
    Ok(())
}

pub fn stop_audio_playback(player: &mut Sink) -> Result<(), anyhow::Error> {
    player.stop();
    Ok(())
}
