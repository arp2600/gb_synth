use rand;

pub struct NoiseOsc {
    table: [f32; 44100],
    table_index: usize,
    amplitude: f32,
}

impl NoiseOsc {
    pub fn new() -> NoiseOsc {
        let table = {
            let mut t = [0f32; 44100];
            for v in t.iter_mut() {
                *v = rand::random::<f32>() * 2.0 - 1.0;
            }
            t
        };

        let table_index = 0;
        let amplitude = 1.0;
        NoiseOsc {
            table,
            table_index,
            amplitude,
        }
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn tick(&mut self) -> f32 {
        let t = self.table_index;
        self.table_index = (self.table_index + 1) % self.table.len();
        self.table[t] * self.amplitude
    }
}
