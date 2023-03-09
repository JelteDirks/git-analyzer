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
use utils::{output::produce_output, settings::Settings};
use walkdir::WalkDir;
use std::thread;

fn main() {
    let args = Args::parse();

    let settings: Settings = Settings::from_args(args);

    let mut err_handle = BufWriter::new(stderr());

    let mut analytics_list: Vec<Analytic> = Vec::with_capacity(1_000);

    dbg!(&settings);

    let entries = WalkDir::new(&settings.path)
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
    }

    produce_output(analytics_list);

    err_handle.flush().unwrap();
}
