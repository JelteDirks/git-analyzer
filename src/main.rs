mod cli;
mod structures;
mod utils;

use crate::cli::args::Args;

use clap::Parser;
use std::{
    io::{BufRead, Write},
    path::Path,
};
use structures::analytics::Analytic;
use utils::{
    lines::{process_byte_slice, process_stdin_lines},
    output::produce_output,
};

// need to rewrite because this is not supporting multi threading really well
//
// 1) use input from git log to analyze the repo, pipe this into the program
// so that the user can select their own commits based on git log
//
// 2) Distribute the commit hashes over a number of threads to analyze those
// commits independent of the main thread.
//
// 3) Use the analyzed aggregated data to improve show stuff to the user
//
// 4) use an optional setting where the use can just analyze the entire repo,
// using the following formatting for logging

// formatting
// git log --format="%H %ct %ae"
//
// %H is the long hash
// %ct is commit date
// %ae is the author email

fn main() {
    let args = Args::parse();
    std::io::stdout()
        .write(format!("program called with args {:?}\n", args).as_bytes())
        .unwrap();
    let mut analytics_list: Vec<Analytic> = Vec::new();

    if args.stdin {
        // when stdin is used, no special treatment is needed so far
        // expand with detailed analytics later?
        let stdin = std::io::stdin().lock();
        process_stdin_lines(stdin.lines(), &mut analytics_list);
        produce_output(analytics_list, &args);
        std::process::exit(0);
    }

    if args.path.is_some() {
        let project_directory = Path::new(args.path.as_ref().unwrap());
        let cwd = std::env::set_current_dir(project_directory);
        if cwd.is_err() {
            std::io::stderr()
                .write(format!("could not change into {:?}\n", project_directory).as_bytes())
                .unwrap();
            std::process::exit(1);
        }
        cwd.unwrap();

        let cmd = std::process::Command::new("sh")
            .arg("-c")
            .arg("git log -p")
            .output();

        if cmd.is_err() {
            std::io::stderr()
                .write(
                    format!("problem with executing git log -p in the project directory\n")
                        .as_bytes(),
                )
                .unwrap();
            std::io::stderr()
                .write(cmd.unwrap_err().to_string().as_bytes())
                .unwrap();
            std::process::exit(1);
        }

        let stdout = cmd.unwrap().stdout;
        process_byte_slice(stdout.as_slice(), &mut analytics_list);
    }

    produce_output(analytics_list, &args);
}
