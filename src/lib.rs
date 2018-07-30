mod saw;
use saw::SawOsc;

pub fn saw_test() -> Vec<f32> {
    let mut saw = SawOsc::new();
    let mut buffer = vec![0.0; 44100];

    for i in buffer.iter_mut() {
        *i = saw.tick();
    }
    buffer
}
