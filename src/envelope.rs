pub struct Envelope {
    start_amp: f32,
    duration: f32,
    t: u32,
}

impl Envelope {
    pub fn new() -> Envelope {
        let start_amp = 0.0;
        let duration = 0.0;
        let t = 0;
        Envelope {
            start_amp,
            duration,
            t,
        }
    }

    pub fn reset(&mut self) {
        self.t = 0;
    }

    pub fn set_start_amp(&mut self, amp: f32) {
        self.start_amp = amp;
    }

    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }

    pub fn tick(&mut self) -> f32 {
        let t = self.t as f32 / 44100.0;
        if t < self.duration {
            let t = t / self.duration;
            let amp = (1.0 - t) * self.start_amp;
            self.t += 1;
            amp
        } else {
            0.0
        }
    }
}
