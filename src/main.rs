mod cli;
mod structures;
mod utils;

use crate::cli::args::Args;

use clap::Parser;
use std::{
    io::{stderr, stdin, stdout, BufRead, Write},
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
    let mut err_handle = stderr();
    let mut analytics_list: Vec<Analytic> = Vec::new();

    if args.stdin {
        // when stdin is used, no special treatment is needed so far
        // expand with detailed analytics later?
        let stdin = stdin().lock();
        process_stdin_lines(stdin.lines(), &mut analytics_list);
        produce_output(analytics_list, &args);
        exit(0);
    }

    let command = match &args.command {
        Some(c) => c.to_string(),
        None => "git log -p".to_string(),
    };

    write!(stdout(), "using command '{}'\n", command).unwrap();

    if args.path.is_some() {
        let project_directory = Path::new(args.path.as_ref().unwrap());
        let cwd = std::env::set_current_dir(project_directory);

        if cwd.is_err() {
            write!(
                err_handle,
                "could not change directory to {:?}\n",
                project_directory
            )
            .unwrap();
            exit(1);
        } else {
            write!(stdout(), "analyzing '{}'\n", project_directory.display()).unwrap();
        }
    }

    let cmd = std::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output();

    if cmd.is_err() {
        write!(
            err_handle,
            "problem with executing '{}' in the working directory chosen\n",
            command
        )
        .unwrap();
        write!(
            err_handle,
            "{:?}\n",
            cmd.unwrap_err().to_string().as_bytes()
        )
        .unwrap();
        err_handle.flush().unwrap();
        exit(1);
    }

    let cmd_stdo = &cmd.as_ref().unwrap().stdout;
    let cmd_stde = &cmd.as_ref().unwrap().stderr;

    if cmd_stde.len() > 0 {
        write!(err_handle, "command '{}' produced errors:\n", command).unwrap();
        write!(err_handle, "{:?}\n", std::str::from_utf8(cmd_stde).unwrap()).unwrap();
        exit(1);
    }

    if cmd_stdo.len() == 0 {
        write!(err_handle, "command '{}' produced no output\n", command).unwrap();
        exit(1);
    }

    process_byte_slice(cmd_stdo.as_slice(), &mut analytics_list);

    produce_output(analytics_list, &args);
}
