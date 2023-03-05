mod cli;
mod structures;
mod utils;

use crate::cli::args::Args;

use clap::Parser;
use std::{
    fs::DirEntry,
    io::{stderr, stdin, stdout, BufRead, BufWriter, Write},
    path::{Path, PathBuf},
    process::exit,
};
use structures::analytics::Analytic;
use utils::{
    lines::{process_byte_slice, process_stdin_lines},
    output::produce_output,
};
use walkdir::WalkDir;

fn main() {
    let args = Args::parse();
    let mut err_handle = BufWriter::new(stderr());
    let mut out_stream = BufWriter::new(stdout());

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

    let path = match args.path.as_ref() {
        Some(p) => PathBuf::from(p),
        None => std::env::current_dir().expect("problem getting current dir"),
    };

    let depth: u32 = match args.depth {
        Some(d) => d,
        None => 0,
    };

    let entries = WalkDir::new(&path)
        .min_depth(depth as usize)
        .max_depth(depth as usize);

    for entry in entries {

        write!(
            err_handle,
            "checked {}\n",
            entry.as_ref().unwrap().path().display()
        )
        .unwrap();

        let cd = std::env::set_current_dir(&entry.as_ref().unwrap().path());

        if cd.is_err() {
            write!(
                err_handle,
                "can not analyze {}: {}",
                &entry.as_ref().unwrap().path().display(),
                cd.err().unwrap().to_string()
            ).unwrap();
            continue;
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
            write!(
                err_handle,
                "command '{}' produced errors in {}:\n",
                command,
                entry.as_ref().unwrap().path().display()
            )
            .unwrap();
            write!(err_handle, "{:?}\n", std::str::from_utf8(cmd_stde).unwrap()).unwrap();
            continue;
        }

        if cmd_stdo.len() == 0 {
            write!(
                err_handle,
                "command '{}' produced no output in {}\n",
                command,
                entry.as_ref().unwrap().path().display()
            )
            .unwrap();
            err_handle.flush().unwrap();
            continue;
        }

        process_byte_slice(cmd_stdo.as_slice(), &mut analytics_list);
    }

    produce_output(analytics_list, &args);

    err_handle.flush().unwrap();
}
