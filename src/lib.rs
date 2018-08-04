extern crate cpal;
extern crate crossbeam_deque;
extern crate hound;
mod pulse;
use cpal::EventLoop;
use cpal::{StreamData, UnknownTypeOutputBuffer};
use crossbeam_deque as deque;
use crossbeam_deque::{Stealer, Worker};
use std::cell::RefCell;
// use std::f32::consts::PI;
use pulse::PulseOsc;
use std::i16;
use std::thread;

pub struct SynthController<'a> {
    server: RefCell<&'a mut AudioServer>,
}

enum ControlAction {
    SetFrequency { channel: u8, frequency: f32 },
    SetAmplitude { channel: u8, amplitude: f32 },
}

impl<'a> SynthController<'a> {
    pub fn set_freq(&mut self, channel: u8, frequency: f32) {
        let action = ControlAction::SetFrequency { channel, frequency };
        self.server.borrow_mut().worker.push(action);
    }

    pub fn set_amp(&mut self, channel: u8, amplitude: f32) {
        let action = ControlAction::SetAmplitude { channel, amplitude };
        self.server.borrow_mut().worker.push(action);
    }
}

pub struct AudioServer {
    worker: Worker<ControlAction>,
}

impl AudioServer {
    pub fn new() -> AudioServer {
        let (worker, s) = deque::lifo();

        thread::spawn(move || {
            run_audio(s);
        });

        AudioServer { worker }
    }

    pub fn get_synth_controller(&mut self) -> SynthController {
        SynthController {
            server: RefCell::new(self),
        }
    }
}

// struct SinOsc {
//     frequency: f32,
//     phase: f32,
// }
//
// impl SinOsc {
//     fn new() -> SinOsc {
//         SinOsc { frequency: 0.0, phase: 0.0 }
//     }
//
//     fn tick(&mut self) -> f32 {
//         let inc = self.frequency / 44100.0;
//         self.phase += inc;
//         (self.phase * PI * 2.0).sin()
//     }
// }

fn run_audio(stealer: Stealer<ControlAction>) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("output.wav", spec).unwrap();

    let event_loop = EventLoop::new();
    let device = cpal::default_output_device().unwrap();

    let format = {
        let mut formats = device.supported_output_formats().unwrap();
        formats.next().unwrap().with_max_sample_rate()
    };
    println!("Using format {:?}", format);

    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id);

    let mut chan1 = PulseOsc::new();
    let mut chan2 = PulseOsc::new();
    let mut chan3 = PulseOsc::new();
    // let mut sin = SinOsc::new();
    // sin.frequency = 440.0;

    let mut chan = 0;
    event_loop.run(move |_stream_id, stream_data| {
        // let count = match &stream_data {
        //     StreamData::Output { buffer } => buffer.len(),
        //     _ => panic!("something something dark side"),
        // };

        while let Some(action) = stealer.steal() {
            match action {
                ControlAction::SetFrequency { channel, frequency } => match channel {
                    1 => chan1.set_frequency(frequency),
                    2 => chan2.set_frequency(frequency),
                    3 => chan3.set_frequency(frequency),
                    _ => (),
                },
                ControlAction::SetAmplitude { channel, amplitude } => match channel {
                    1 => chan1.set_amplitude(amplitude),
                    2 => chan2.set_amplitude(amplitude),
                    3 => chan3.set_amplitude(amplitude),
                    _ => (),
                },
            }
        }

        match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                let channels = format.channels as usize;
                for sample in buffer.chunks_mut(channels) {
                    let mut x = 0.0;
                    x += chan1.tick();
                    x += chan2.tick();
                    // x += chan3.tick();
                    let x = x * 0.1;
                    for out in sample.iter_mut() {
                        *out = x;
                    }

                    let x = x.min(1.0).max(-1.0);
                    let amplitude = i16::MAX as f32;
                    writer.write_sample((x * amplitude) as i16).unwrap();
                }
                writer.flush().unwrap();
            }
            _ => panic!("Unsupported stream data type"),
        }

        chan += 1;
    });
}
