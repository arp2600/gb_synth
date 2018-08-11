pub struct WavetableOsc {
    frequency: f32,
    phase_accumulator: f32,
    amplitude: f32,
    table: [f32; 32],
}

impl WavetableOsc {
    pub fn new() -> WavetableOsc {
        let frequency = 440.0;
        let phase_accumulator = 0.0;
        let amplitude = 0.0;
        let mut table = [0.0; 32];

        for v in table.iter_mut().take(32 / 2) {
            *v = 1.0;
        }
        for v in table.iter_mut().skip(32 / 2) {
            *v = -1.0;
        }

        WavetableOsc {
            frequency,
            phase_accumulator,
            amplitude,
            table,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn tick(&mut self) -> f32 {
        let table_len = self.table.len() as f32;
        let inc = (table_len * self.frequency) / 44100.0;

        let phase = {
            let p = self.phase_accumulator;
            self.phase_accumulator = p + inc;
            if self.phase_accumulator >= table_len {
                self.phase_accumulator -= table_len;
            }
            p
        };

        self.table[phase as usize] * self.amplitude
    }
}
