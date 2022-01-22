use std::thread;
use std::time::{Duration, Instant};

fn main() {
    measure_the_elapsed_time_between_two_code_sections();
}

fn expensive_function() {
    thread::sleep(Duration::from_secs(1));
}

fn measure_the_elapsed_time_between_two_code_sections() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("Time elapsed in expensive function() is: {:?}", duration);
}
