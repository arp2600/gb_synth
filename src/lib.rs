mod saw;
mod pulse;
use saw::SawOsc;
use pulse::PulseOsc;

pub fn saw_test() -> Vec<f32> {
    let mut saw = PulseOsc::new();
    let mut buffer = vec![0.0; 44100];

    let mut max_v = 0.0;
    for i in buffer.iter_mut() {
        *i = saw.tick();
        max_v = (*i).abs().max(max_v);
    }

    // Normalize buffer
    for i in buffer.iter_mut() {
        *i = *i / max_v;
    }
    println!("max_v = {}", max_v);
    buffer
}
