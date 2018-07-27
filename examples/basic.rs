extern crate cpal;
use cpal::EventLoop;
use cpal::{OutputBuffer, StreamData, UnknownTypeOutputBuffer};

struct PulseOsc {
    frequency: u32,
    counter: u32,
}

impl PulseOsc {
    fn tick(&mut self, buffer: &mut [f32]) {
        let x = 44100 / self.frequency;
        for sample in buffer.iter_mut() {
            let value = (self.counter / x) % 2;
            let value = (value as f32) * 2.0 - 1.0;
            *sample = value;
            self.counter += 1;
        }
    }
}

fn main() {
    println!("Available outputs:");
    for device in cpal::output_devices() {
        println!("    {}", device.name());
    }
    println!("");

    println!("Default output:");
    println!("    {}", cpal::default_output_device().unwrap().name());
    println!("");

    let event_loop = EventLoop::new();
    let device = cpal::default_output_device().unwrap();

    let format = {
        let mut formats = device.supported_output_formats().unwrap();
        formats.next().unwrap().with_max_sample_rate()
    };
    println!("{:?}", format);

    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id);

    let mut pulse = PulseOsc {
        frequency: 440,
        counter: 0,
    };

    event_loop.run(move |_stream_id, stream_data| {
        let count = match &stream_data {
            StreamData::Output { buffer } => buffer.len(),
            _ => panic!("something something dark side"),
        };

        match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                pulse.tick(&mut buffer);
                // for elem in buffer.iter_mut() {
                //     *elem = 0.0;
                // }
            }
            _ => panic!("Unsupported stream data type"),
        }
    });
}
