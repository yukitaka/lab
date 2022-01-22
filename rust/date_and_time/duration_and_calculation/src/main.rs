use std::thread;
use std::time::{Duration, Instant};

mod use_chrono;

fn main() {
    measure_the_elapsed_time_between_two_code_sections();
    use_chrono::perform_checked_date_and_time_calculations();
    use_chrono::convert_a_local_time_to_another_timezone();
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
