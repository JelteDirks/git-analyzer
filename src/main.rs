mod cli;
mod structures;
mod utils;

use crate::cli::args::Args;

use clap::Parser;
use utils::lines::process_byte_slice;
use std::process::Command;
use std::thread;
use std::{
    io::{stderr, BufWriter, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
};
use structures::analytics::Analytic;
use utils::{output::produce_output, settings::Settings};
use walkdir::WalkDir;

fn main() {
    let args = Args::parse();

    let s = Box::new(Settings::from_args(args));
    let settings: &'static Settings = Box::leak(s);

    let mut err_handle = BufWriter::new(stderr());
    let mut arc_list: Arc<Mutex<Vec<Analytic>>> = Arc::new(Mutex::new(Vec::with_capacity(1_000)));

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

        let anset = AnalyzeSettings::build(path_buf, Arc::clone(&arc_list), settings);

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
    let mut err_handle = stderr();

    let cd = std::env::set_current_dir(anset.path);
    if cd.is_err() {
        write!(err_handle, "{}\n", cd.err().unwrap()).unwrap();
        return;
    }

    let output = Command::new("sh").arg("-c").arg(&anset.settings.command).output();
    if output.is_err() {
        write!(err_handle, "{}\n", output.err().unwrap()).unwrap();
        return;
    }

    let mut local: Vec<Analytic> = Vec::with_capacity(100);

    process_byte_slice(&output.unwrap().stdout, &mut local, anset.settings);

    let mut locked_list = anset.list.lock().expect("error inside critical section");
    locked_list.append(&mut local);
}

#[derive(Debug)]
struct AnalyzeSettings<'a> {
    path: PathBuf,
    list: Arc<Mutex<Vec<Analytic>>>,
    settings: &'a Settings,
}

impl<'a> AnalyzeSettings<'a> {
    fn build(path: PathBuf, list: Arc<Mutex<Vec<Analytic>>>, settings: &'a Settings) -> Self {
        return AnalyzeSettings {
            path,
            list,
            settings,
        };
    }
}
