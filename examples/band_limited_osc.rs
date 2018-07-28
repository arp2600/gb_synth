extern crate hound;
extern crate rustplotlib;
use rustplotlib::Figure;
use std::f32::consts::PI;
use std::i16;

struct PhaseAccumulator {
    current_phase: f32,
}

impl PhaseAccumulator {
    fn new() -> PhaseAccumulator {
        let current_phase = 0.0;
        PhaseAccumulator { current_phase }
    }

    fn tick(&mut self, inc: f32, amp: f32) -> f32 {
        let phase = self.current_phase;

        let new_phase = self.current_phase + inc;
        self.current_phase = wrap(new_phase, amp);

        phase
    }
}

struct BLIT {
    max_freq: f32,
}

impl BLIT {
    fn new() -> BLIT {
        // let max_freq = 18000.0;
        let max_freq = (44100.0 / 2.0) * 0.95;
        BLIT { max_freq }
    }

    fn tick(&self, phase: f32, frequency: f32) -> f32 {
        if phase == 0.0 {
            1.0
        } else {
            let n = (self.max_freq / frequency).floor();
            let x = (phase * (n + 0.5)).sin();
            let y = (phase / 2.0).sin();
            0.5 * ((x / y) - 1.0)
        }
    }
}

struct LeakyIntegrator {
    feedback: f32,
}

impl LeakyIntegrator {
    fn tick(&mut self, blit: f32, inc: f32) -> f32 {
        let inc = inc * 0.25;
        let b = blit - self.feedback;
        let x = inc * b;
        let x = x + self.feedback;
        self.feedback = x;
        x
    }
}

fn wrap(v: f32, n: f32) -> f32 {
    let v = v * (1.0 / n);
    let v = v - v.round();
    v * n
}

fn make_figure<'a>(x: &'a [f64], buffer: &'a [f64]) -> Figure<'a> {
    use rustplotlib::{Axes2D, Line2D};

    let ax1 = Axes2D::new().add(Line2D::new(r"$y_1 = \sin(x)$").data(x, buffer));

    Figure::new().subplots(2, 1, vec![Some(ax1)])
}

fn main() {
    let frequency = 440.0 * 3.0;
    // let frequency = 2000.0;
    let n = PI;
    let n = n * 2.0;
    let f = (n * frequency) / 44100.0;

    let mut pa = PhaseAccumulator::new();
    let blit = BLIT::new();
    let mut li = LeakyIntegrator { feedback: 0.0 };
    let mut buffer = Vec::new();

    // wav file writer
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    // create the saw wav
    for _ in 0..(44100 * 5) {
        let x = pa.tick(f, n);
        let x = blit.tick(x, frequency);
        let x = li.tick(x, f);
        let amplitude = i16::MAX as f32;
        writer.write_sample((x * amplitude) as i16).unwrap();
        buffer.push(x as f64);
    }

    let x: Vec<_> = (0..buffer.len()).map(|x| x as f64).collect();
    // let buffer: Vec<_> = buffer.iter().map(|x| *x as f64).collect();
    let fig = make_figure(&x, &buffer);

    use rustplotlib::backend::Matplotlib;
    use rustplotlib::Backend;
    let mut mpl = Matplotlib::new().unwrap();
    mpl.set_style("ggplot").unwrap();

    fig.apply(&mut mpl).unwrap();

    mpl.savefig("simple.png").unwrap();
    mpl.dump_pickle("simple.fig.pickle").unwrap();
    mpl.wait().unwrap();
}
