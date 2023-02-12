mod cli;
mod structures;
mod utils;

use crate::cli::args::Args;

use clap::Parser;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Lines, StdinLock, Write},
};
use structures::analytics::Analytic;

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
    let mut analytics_list: Vec<Analytic> = Vec::new();

    if args.stdin {
        let stdin = std::io::stdin().lock();
        process_stdin_lines(stdin.lines(), &mut analytics_list);
    } else if args.path.is_some() {
        // do a git log -p in that path
        // analyze the results of that log
    }

    produce_output(analytics_list);
}

fn produce_output(analytics_list: Vec<Analytic>) {
    let mut analytics_collection: HashMap<String, Analytic> = HashMap::new();

    for a in analytics_list {
        let key = a.extension.as_ref().unwrap();
        analytics_collection
            .entry(key.into())
            .and_modify(|existing| {
                existing.additions += a.additions;
                existing.deletions += a.deletions;
            })
            .or_insert(a);
    }

    let mut stdout = std::io::stdout();

    for a in analytics_collection.iter() {
        let (extension, analytic) = a;
        stdout
            .write(format!("For {} files\n", extension).as_bytes())
            .unwrap();
        stdout
            .write(format!("\t{} additions\n", analytic.additions).as_bytes())
            .unwrap();
        stdout
            .write(format!("\t{} deletions\n", analytic.deletions).as_bytes())
            .unwrap();
    }
}

enum AnalyzeState {
    DiffLine,
    Changes,
}

use crate::utils::lines::{find_extension_from_diff, is_addition, is_deletion, is_diff_line};

fn process_stdin_lines<'a>(
    lines: Lines<StdinLock>,
    analytics_list: &'a mut Vec<Analytic>,
) -> &'a mut Vec<Analytic> {
    let mut state = AnalyzeState::DiffLine;
    let mut analytic = Analytic::default();

    for line in lines {
        if line.is_err() {
            todo!("error in the line from stdin, handle it gracefully");
        }
        match state {
            AnalyzeState::DiffLine => {
                if is_diff_line(&line) {
                    state = AnalyzeState::Changes;
                    let ext = find_extension_from_diff(&line.unwrap().as_bytes());
                    analytic.extension = Some(ext.into());
                }
            }
            AnalyzeState::Changes => {
                if is_diff_line(&line) {
                    analytics_list.push(analytic);
                    analytic = Analytic::default();
                    let ext = find_extension_from_diff(&line.unwrap().as_bytes());
                    analytic.extension = Some(ext.into());
                    // TODO: do the saving of this analytic in here and continue
                    // with the new diff to analyze
                    continue;
                }
                if is_addition(&line) {
                    analytic.additions += 1;
                } else if is_deletion(&line) {
                    analytic.deletions += 1;
                }
            }
        }
    }
    analytics_list.push(analytic);
    return analytics_list;
}
