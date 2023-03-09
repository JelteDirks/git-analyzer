mod cli;
mod structures;
mod utils;

use crate::cli::args::Args;

use clap::Parser;
use std::{
    io::{stderr, BufWriter, Write},
    path::PathBuf, sync::{Mutex, Arc},
};
use structures::analytics::Analytic;
use utils::{output::produce_output, settings::Settings};
use walkdir::WalkDir;
use std::thread;

fn main() {
    let args = Args::parse();

    let s = Box::new(Settings::from_args(args));
    let settings = Box::leak(s);

    let mut err_handle = BufWriter::new(stderr());

    let mut analytics_list: Vec<Analytic> = Vec::with_capacity(1_000);
    let mut arc_list = Arc::new(Mutex::new(analytics_list));

    dbg!(&settings);

    let entries = WalkDir::new(&settings.base)
        .min_depth(settings.depth as usize)
        .max_depth(settings.depth as usize);

    let mut thread_handles = Vec::new();

    for entry in entries.into_iter() {
        if entry.is_err() {
            write!(err_handle, "{}\n", entry.as_ref().err().unwrap()).unwrap();
            continue;
        }

        let path_buf: PathBuf = entry.unwrap().path().to_path_buf();

        if !path_buf.is_dir() {
            continue;
        }

        let anset = AnalyzeSettings::build(path_buf, &settings.command, Arc::clone(&arc_list));

        let t_handle = thread::spawn(move || {
            analyze(anset);
        });

        thread_handles.push(t_handle);
    }

    for t_handle in thread_handles.into_iter() {
        t_handle.join().unwrap();
    }

    err_handle.flush().unwrap();
}

fn analyze(anset: AnalyzeSettings) {
}

#[derive(Debug)]
struct AnalyzeSettings<'a> {
    path: PathBuf,
    command: &'a str,
    list: Arc<Mutex<Vec<Analytic>>>,
}

impl<'a> AnalyzeSettings<'a> {
    fn build(path: PathBuf, command: &'a str, list: Arc<Mutex<Vec<Analytic>>>) -> Self {
        return AnalyzeSettings {
            path,
            command,
            list,
        }
    }
}
