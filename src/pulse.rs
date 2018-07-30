use std::f32::consts::PI;

pub struct PulseOsc {
    frequency: f32,
    sample_rate: f32,
    phase_accumulator: f32,
    integrator_feedback: f32,
}

impl PulseOsc {
    pub fn new() -> PulseOsc {
        let frequency = 440.0;
        let sample_rate = 44100.0;
        let phase_accumulator = 0.0;
        let integrator_feedback = 0.0;

        PulseOsc {
            frequency,
            sample_rate,
            phase_accumulator,
            integrator_feedback,
        }
    }

    pub fn tick(&mut self) -> f32 {
        let inc = (PI * 2.0 * self.frequency) / self.sample_rate;

        let (phase_1, phase_2) = {
            let p = self.phase_accumulator;
            self.phase_accumulator = wrap(p + inc, 2.0 * PI);
            (p, wrap(p + PI, 2.0 * PI))
        };

        let blit_value = {
            let max_freq = (self.sample_rate / 2.0) * 0.95;
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
        li
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
    let v = v * (1.0 / n);
    let v = v - v.round();
    v * n
}
