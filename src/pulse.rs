use std::f32::consts::PI;

pub struct PulseOsc {
    frequency: f32,
    phase_accumulator: f32,
    integrator_feedback: f32,
    amplitude: f32,
}

impl PulseOsc {
    pub fn new() -> PulseOsc {
        let frequency = 440.0;
        let phase_accumulator = 0.0;
        let integrator_feedback = 0.0;

        PulseOsc {
            frequency,
            phase_accumulator,
            integrator_feedback,
            amplitude: 1.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn tick(&mut self) -> f32 {
        let inc = (PI * 2.0 * self.frequency) / 44100.0;

        let (phase_1, phase_2) = {
            let p = self.phase_accumulator;
            self.phase_accumulator = wrap(p + inc, PI);
            (p, wrap(p + PI, PI))
        };

        let blit_value = {
            let max_freq = (44100.0 / 2.0) * 0.95;
            let blit_value_1 = blit(phase_1, self.frequency, max_freq);
            let blit_value_2 = blit(phase_2, self.frequency, max_freq);
            blit_value_1 - blit_value_2
        };

        let li = {
            let b = blit_value - self.integrator_feedback;
            let x = inc * 0.25 * b;
            let x = x + self.integrator_feedback;
            self.integrator_feedback = x;
            x
        };
        li * self.amplitude
    }
}

fn blit(phase: f32, frequency: f32, max_frequency: f32) -> f32 {
    if phase == 0.0 {
        1.0
    } else {
        let n = (max_frequency / frequency).floor();
        let x = (phase * (n + 0.5)).sin();
        let y = (phase / 2.0).sin();
        0.5 * ((x / y) - 1.0)
    }
}

fn wrap(v: f32, n: f32) -> f32 {
    let n = n * 2.0;
    let v = v * (1.0 / n);
    let v = v - v.round();
    v * n
}

#[cfg(test)]
mod tests {
    extern crate hound;
    use pulse::PulseOsc;
    use std::i16;

    #[test]
    fn sometest() {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

        let mut pulse = PulseOsc::new();
        for _ in 0..(44100 * 5) {
            let x = pulse.tick() * 0.5;
            let x = x.min(1.0).max(-1.0);
            let amplitude = i16::MAX as f32;
            writer.write_sample((x * amplitude) as i16).unwrap();
        }
    }
}
