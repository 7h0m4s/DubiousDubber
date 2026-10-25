use audio_recorder_rs::Recorder;
use cpal::traits::{DeviceTrait, HostTrait};
use std::thread;


pub fn start_audio_recording(mut recorder: &mut Recorder, filename: &str)-> Result<(),anyhow::Error> {



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
            sample_format: if sample_format.is_float() {hound::SampleFormat::Float} else {hound::SampleFormat::Int},
        };
        let mut writer = hound::WavWriter::create(filename, spec)?;


        let receiver = recorder.start(true).expect("Failed to start audio recording");

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








    // fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
    // where
    //     T: Sample,
    //     U: Sample + hound::Sample + FromSample<T>,
    // {
    //     if let Ok(mut guard) = writer.try_lock() {
    //         if let Some(writer) = guard.as_mut() {
    //             for &sample in input.iter() {
    //                 let sample: U = U::from_sample(sample);
    //                 writer.write_sample(sample).ok();
    //             }
    //         }
    //     }
    // }

    // fn write_input_data2<T, U>(data: &[T], writer: &Arc<Mutex<WavWriter<BufWriter<File>>>>)
    // where
    //     T: Sample,
    //     U: Sample + hound::Sample + FromSample<T>,
    // {
    //     let mut writer = writer.lock().unwrap();
    //     for &sample in data {
    //         writer.write_sample(sample.to_sample::<U>()).ok();
    //     }
    // }




