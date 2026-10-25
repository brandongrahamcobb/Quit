use std::io::Error;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Error> {
    Ok(())
}
