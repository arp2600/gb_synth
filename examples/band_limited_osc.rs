extern crate gb_synth;
extern crate hound;
extern crate rustplotlib;
use rustplotlib::Figure;
use std::f32::consts::PI;
use std::i16;

fn make_figure<'a>(x: &'a [f64], buffer: &'a [f64]) -> Figure<'a> {
    use rustplotlib::{Axes2D, Line2D};

    let ax1 = Axes2D::new().add(Line2D::new(r"$y_1 = \sin(x)$").data(x, buffer));

    Figure::new().subplots(2, 1, vec![Some(ax1)])
}

fn main() {
    // wav file writer
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let buffer = gb_synth::saw_test();
    for x in buffer.iter() {
        let amplitude = i16::MAX as f32;
        writer.write_sample((*x * amplitude) as i16).unwrap();
    }

    let x: Vec<_> = (0..buffer.len()).map(|x| x as f64).collect();
    let buffer: Vec<_> = buffer.iter().map(|x| *x as f64).collect();
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
