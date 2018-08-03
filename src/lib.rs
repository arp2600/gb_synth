extern crate cpal;
extern crate crossbeam_deque;
use cpal::EventLoop;
use cpal::{OutputBuffer, StreamData, UnknownTypeOutputBuffer};
use crossbeam_deque as deque;
use crossbeam_deque::{Stealer, Worker};
use std::cell::RefCell;
use std::thread;
use std::f32::consts::PI;

pub struct SynthController<'a> {
    server: RefCell<&'a mut AudioServer>,
}

enum ControlAction {
    SetFrequency { channel: u8, frequency: f32 },
}

impl<'a> SynthController<'a> {
    pub fn set_freq(&mut self, channel: u8, frequency: f32) {
        let action = ControlAction::SetFrequency { channel, frequency };
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

struct SinOsc {
    frequency: f32,
    phase: f32,
}

impl SinOsc {
    // fn tick(&mut self, buffer: &mut [f32]) {
    //     let inc = self.frequency / 44100.0;
    //     for sample in buffer.iter_mut() {
    //         self.phase += inc;
    //         *sample = (self.phase * PI).sin() * 0.1;
    //     }
    // }

    fn tick(&mut self) -> f32 {
        let inc = self.frequency / 44100.0;
        self.phase += inc;
        (self.phase * PI).sin()
    }
}

fn wrap(v: f32, n: f32) -> f32 {
    let v = v * (1.0 / n);
    let v = v - v.round();
    v * n
}

fn run_audio(stealer: Stealer<ControlAction>) {
    let event_loop = EventLoop::new();
    let device = cpal::default_output_device().unwrap();

    let format = {
        let mut formats = device.supported_output_formats().unwrap();
        formats.next().unwrap().with_max_sample_rate()
    };

    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id);

    let mut chan1 = SinOsc {
        frequency: 440.0,
        phase: 0.0,
    };

    let mut chan2 = SinOsc {
        frequency: 440.0,
        phase: 0.0,
    };

    let mut chan3 = SinOsc {
        frequency: 440.0,
        phase: 0.0,
    };

    event_loop.run(move |_stream_id, stream_data| {
        let count = match &stream_data {
            StreamData::Output { buffer } => buffer.len(),
            _ => panic!("something something dark side"),
        };

        while let Some(action) = stealer.steal() {
            match action {
                ControlAction::SetFrequency { channel, frequency } => {
                    match channel {
                        1 => chan1.frequency = frequency,
                        2 => chan2.frequency = frequency,
                        3 => chan3.frequency = frequency,
                        _ => (),
                    }
                },
            }
        }

        match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                for v in buffer.iter_mut() {
                    *v = (chan1.tick() + chan2.tick() + chan3.tick()) * 0.1;
                }
            }
            _ => panic!("Unsupported stream data type"),
        }
    });
}
