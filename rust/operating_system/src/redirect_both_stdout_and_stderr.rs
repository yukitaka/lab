use std::fs::File;
use std::io::Error;
use std::process::{Command, Stdio};

pub fn redirect_both_stdout_and_stderr_of_child_process_to_the_same_file() -> Result<(), Error> {
    let outputs = File::create("out.txt")?;
    let errors = outputs.try_clone()?;

    Command::new("ls")
        .args(&[".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;
    Ok(())
}
