use std::f32::consts::PI;

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

struct SawOsc {
    frequency: f32,
    inc: f32,
    pa: PhaseAccumulator,
    blit: BLIT,
    li: LeakyIntegrator,
}

impl SawOsc {
    fn new() -> SawOsc {
        let pa = PhaseAccumulator::new();
        let blit = BLIT::new();
        let li = LeakyIntegrator { feedback: 0.0 };

        let mut saw = SawOsc {
            frequency: 0.0,
            inc: 0.0,
            pa,
            blit,
            li,
        };
        saw.set_frequency(440.0);
        saw
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.inc = (PI * 2.0 * self.frequency) / 44100.0;
    }

    fn tick(&mut self, buffer: &mut [f32]) {
        for v in buffer.iter_mut() {
            let x = self.pa.tick(self.inc, PI * 2.0);
            let x = self.blit.tick(x, self.frequency);
            let x = self.li.tick(x, self.inc);
            *v = x;
        }
    }
}

fn wrap(v: f32, n: f32) -> f32 {
    let v = v * (1.0 / n);
    let v = v - v.round();
    v * n
}

pub fn saw_test() -> Vec<f32> {
    let mut saw = SawOsc::new();
    let mut buffer = vec![0.0; 44100];

    saw.tick(&mut buffer);
    buffer
}
