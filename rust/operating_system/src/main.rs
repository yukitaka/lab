mod process_stdout;

fn main() {
    if let Err(e) = process_stdout::run_an_external_command_and_process_stdout() {
        println!("{}", e);
    }
}
