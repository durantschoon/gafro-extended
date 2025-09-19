mod json_loader;
mod test_runner;

use clap::Parser;
use test_runner::{Args, run_tests};

fn main() {
    let args = Args::parse();
    
    match run_tests(args) {
        Ok(exit_code) => std::process::exit(exit_code),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
