mod cli;
mod structures;
mod utils;

use crate::cli::args::Args;

use clap::Parser;
use std::{
    io::{stderr, BufWriter, Write},
    path::Path,
};
use structures::analytics::Analytic;
use utils::{lines::process_byte_slice, output::produce_output, settings::Settings};
use walkdir::WalkDir;

fn main() {
    let args = Args::parse();

    let settings: Settings = Settings::from_args(args);

    let mut err_handle = BufWriter::new(stderr());

    let mut analytics_list: Vec<Analytic> = Vec::new();

    dbg!(&settings);

    let canon = std::fs::canonicalize(&settings.path);

    let entries = WalkDir::new(canon.unwrap())
        .min_depth(settings.depth as usize)
        .max_depth(settings.depth as usize);

    for entry in entries.into_iter() {
        if entry.is_err() {
            write!(err_handle, "{}\n", entry.as_ref().err().unwrap()).unwrap();
            continue;
        }

        let path_ref: &Path = entry.as_ref().unwrap().path();

        if !path_ref.is_dir() {
            continue;
        }

        let cd = std::env::set_current_dir(path_ref);

        if cd.is_err() {
            write!(
                err_handle,
                "can not analyze {}: {}\n",
                path_ref.display(),
                cd.err().unwrap().to_string()
            )
            .unwrap();
            continue;
        }

        let cmd = std::process::Command::new("sh")
            .arg("-c")
            .arg(&settings.command)
            .output();

        if cmd.is_err() {
            write!(
                err_handle,
                "problem with executing '{}' in the working directory chosen: ",
                &settings.command
            ) .unwrap();
            write!(err_handle, "{}\n", cmd.unwrap_err().to_string()).unwrap();
            continue;
        }

        let cmd_stdo = &cmd.as_ref().unwrap().stdout;
        let cmd_stde = &cmd.as_ref().unwrap().stderr;

        if cmd_stde.len() > 0 {
            write!(
                err_handle,
                "command '{}' produced errors in {}:\n",
                &settings.command,
                path_ref.display()
            )
            .unwrap();
            write!(err_handle, "{:?}\n", std::str::from_utf8(cmd_stde).unwrap()).unwrap();
            continue;
        }

        if cmd_stdo.len() == 0 {
            write!(
                err_handle,
                "command '{}' produced no output in {}\n",
                &settings.command,
                path_ref.display()
            )
            .unwrap();
            err_handle.flush().unwrap();
            continue;
        }

        process_byte_slice(cmd_stdo.as_slice(), &mut analytics_list, &settings);

        write!(err_handle, "checked {}\n", path_ref.display()).unwrap();
    }

    produce_output(analytics_list);

    err_handle.flush().unwrap();
}
