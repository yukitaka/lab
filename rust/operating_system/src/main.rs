mod piped_external_commands;
mod process_stdout;
mod stdin_and_check;

fn main() {
    if let Err(e) = process_stdout::run_an_external_command_and_process_stdout() {
        println!("{}", e);
    }
    if let Err(e) =
        stdin_and_check::run_an_external_command_passing_it_stdin_and_check_for_an_error_code()
    {
        println!("{}", e);
    }
    if let Err(e) = piped_external_commands::run_piped_external_commands() {
        println!("{}", e);
    }
}
