use cat::run;
use std::time::Instant;

fn main() {
    let started_at = Instant::now();
    run();
    elapsed_time(started_at);
}

fn elapsed_time(started: Instant) {
    let elapsed = Instant::now().duration_since(started);

    print!("seconds: {:0.9}", elapsed.as_secs_f64());
}







