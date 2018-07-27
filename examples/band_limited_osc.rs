struct PhaseAccumulator {
    sample_rate: f32,
    current_phase: f32,
    frequency: f32,
    amp: f32,
}

impl PhaseAccumulator {
    fn new() -> PhaseAccumulator {
        let sample_rate = 44100.0;
        let current_phase = 0.0;
        let frequency = 440.0;
        let amp = 1.0;
        PhaseAccumulator {
            sample_rate,
            current_phase,
            frequency,
            amp,
        }
    }

    fn tick(&mut self) -> f32 {
        let phase = self.current_phase;

        let amp = self.amp * 2.0;
        let inc = (self.frequency * amp) / self.sample_rate;

        let new_phase = self.current_phase + inc;
        self.current_phase = wrap(new_phase, amp);

        phase
    }

    fn iter_mut(&mut self) -> PhaseAccumulatorIterator {
        PhaseAccumulatorIterator { pa: self }
    }
}

struct PhaseAccumulatorIterator<'a> {
    pa: &'a mut PhaseAccumulator,
}

impl<'a> Iterator for PhaseAccumulatorIterator<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.pa.tick())
    }
}

fn wrap(v: f32, n: f32) -> f32 {
    let v = v * (1.0 / n);
    let v = v - v.round();
    v * n
}

fn main() {
    let mut pa = PhaseAccumulator::new();
    for (i, x) in (0..25).zip(pa.iter_mut()) {
        println!("i: {}, x: {}", i, x);
    }

    println!("taking a break");

    for (i, x) in (25..200).zip(pa.iter_mut()) {
        println!("i: {}, x: {}", i, x);
    }
}
