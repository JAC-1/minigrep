/// # Minigrep
///
/// mini_grep is a learning project from **The Book**.
///
/// Commented out code is what was learned initially. The refactored version is currently in use.
use minigrep::{run, Config};
use std::{env, process};

/// Runs the program using the imported `run()` function and `Config` struct.
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // let args: Vec<String> = env::args().collect();
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
    // run(config).unwrap_or_else(|e| println!("There was a problem reading the file: {e}"));
}
