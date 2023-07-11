//! A basic output stream example, using an Output AudioUnit to generate a sine wave.

extern crate coreaudio;

use coreaudio::audio_unit::render_callback::{self, data};
use coreaudio::audio_unit::{AudioUnit, IOType, SampleFormat};
use std::thread;
use std::sync::mpsc;

mod lfo;
use lfo::LFO;

mod mixer;
mod wave;
mod vco;
mod generator;
mod filter;
use generator::Generator;

type Args = render_callback::Args<data::NonInterleaved<f32>>;


fn main() -> Result<(), coreaudio::Error> {
    let volume = 0.5;
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || -> Result<(), coreaudio::Error> {

        // Construct an Output audio unit that delivers audio to the default output device.
        let mut audio_unit = AudioUnit::new(IOType::DefaultOutput)?;

        // Read the input format. This is counterintuitive, but it's the format used when sending
        // audio data to the AudioUnit representing the output device. This is separate from the
        // format the AudioUnit later uses to send the data to the hardware device.
        let stream_format = audio_unit.input_stream_format()?;
        println!("{:#?}", &stream_format);
         // For this example, our sine wave expects `f32` data.
        assert!(SampleFormat::F32 == stream_format.sample_format);

        audio_unit.start()?;

        loop {
            let recieved = rx.try_recv();
            match recieved {
                Ok(p) => {
                    println!("got message");
                    // let mut filter = filter::Filter::new(1000., 1., 0.);
                    let mut mixer = mixer::Renderer::new(volume);
                    // let mut lfo = LFO::new(20., 50.);
                    let samples = vco::VCO::new(p, 0.8 as f64, wave::sine);

                    let samples2 = vco::VCO::new(55., 0.8 as f64, wave::sine); 
                    mixer.vcos.push(Box::new(samples));
                    mixer.vcos.push(Box::new(samples2));
                    // mixer.vcos.push(Box::new(samples2));
                    // let mut samples2 = wave::Wave::new(300., volume, wave::sine);
                    audio_unit.set_render_callback(move |args| {
                        let Args {
                            num_frames,
                            mut data,
                            ..
                        } = args;
                        for i in 0..num_frames {
                            // let sample = samples.next().unwrap();
                            // let sample2 = samples.next().unwrap();
                            for channel in data.channels_mut() {
                                // channel[i] = filter.next(mixer.out() as f64) as f32;
                                channel[i] = mixer.out()
                            }
                        }
                        Ok::<(), ()>(())
                    })?;
                }
                Err(e) => (),
            };
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });
    
    std::thread::sleep(std::time::Duration::from_millis(1000));
    tx.send(60.).unwrap();
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    _ = handle.join().unwrap();

    Ok(())
}