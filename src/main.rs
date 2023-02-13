mod cli;
mod structures;
mod utils;

use crate::cli::args::Args;

use clap::Parser;
use std::{
    io::{stderr, stdin, BufRead, Write},
    path::Path,
    process::exit,
};
use structures::analytics::Analytic;
use utils::{
    lines::{process_byte_slice, process_stdin_lines},
    output::produce_output,
};

fn main() {
    let args = Args::parse();
    let mut analytics_list: Vec<Analytic> = Vec::new();

    if args.stdin {
        // when stdin is used, no special treatment is needed so far
        // expand with detailed analytics later?
        let stdin = stdin().lock();
        process_stdin_lines(stdin.lines(), &mut analytics_list);
        produce_output(analytics_list, &args);
        exit(0);
    }

    if args.path.is_some() {
        let project_directory = Path::new(args.path.as_ref().unwrap());
        let cwd = std::env::set_current_dir(project_directory);

        if cwd.is_err() {
            stderr()
                .write(format!("could not change into {:?}\n", project_directory).as_bytes())
                .unwrap();
            exit(1);
        }

        let command = match &args.command {
            Some(c) => c.to_string(),
            None => "git log -p".to_string(),
        };

        let cmd = std::process::Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output();

        if cmd.is_err() {
            stderr()
                .write(
                    format!(
                        "problem with executing {} in the project directory\n",
                        command
                    )
                    .as_bytes(),
                )
                .unwrap();
            stderr()
                .write(cmd.unwrap_err().to_string().as_bytes())
                .unwrap();
            exit(1);
        }

        let stdout = cmd.unwrap().stdout;

        if stdout.len() == 0 {
            stderr()
                .write(format!("command {} produced no output\n", command).as_bytes())
                .unwrap();
            exit(1);
        }

        process_byte_slice(stdout.as_slice(), &mut analytics_list);
    }

    produce_output(analytics_list, &args);
}
