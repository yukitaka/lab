mod avoid_discarding_errors;
mod error_scenarios;
mod handle_error;

fn main() {
    handle_error::handle_errors_correctly_in_main();
    avoid_discarding_errors::avoid_discarding_errors_during_error_conversions();
    error_scenarios::obtain_backtrace_of_complex_error_scenarios();
}
