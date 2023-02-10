mod cli;
mod structures;

use crate::cli::args::Args;
use clap::Parser;
use std::io::{BufRead, BufReader, Lines, StdinLock};
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
}

enum AnalyzeState {
    DiffLine,
    MinLine,
    PlusLine,
    Changes,
    Saving,
}

fn process_stdin_lines<'a>(
    lines: Lines<StdinLock>,
    analytics_list: &'a mut Vec<Analytic>,
) -> &'a mut Vec<Analytic> {
    let mut state = AnalyzeState::DiffLine;
    for line in lines {
        if line.is_err() {
            todo!("error in the line from stdin, handle it gracefully");
        }
        match state {
            AnalyzeState::DiffLine => {
                if is_diff_line(&line) {
                    println!("{:?}", line);
                    state = AnalyzeState::MinLine;
                }
            }
            AnalyzeState::MinLine => {
                if is_min_line(&line) {
                    println!("{:?}", line);
                    state = AnalyzeState::DiffLine;
                }
            },
            AnalyzeState::PlusLine => todo!(),
            AnalyzeState::Changes => todo!(),
            AnalyzeState::Saving => todo!(),
        }
    }
    return analytics_list;
}

fn is_min_line(line: &Result<String, std::io::Error>) -> bool {
    let actual = line.as_ref().unwrap();
    if actual.len() < 4 {
        return false;
    }
    if actual.get(0..4).unwrap() == "--- " {
        return true;
    }
    return false;
}

fn is_diff_line(line: &Result<String, std::io::Error>) -> bool {
    let actual = line.as_ref().unwrap();
    if actual.len() < 10 {
        return false;
    }
    if actual.get(0..10).unwrap() == "diff --git" {
        return true;
    }
    return false;
}

fn find_extension_from_diff(diff_line: &[u8]) -> String {
    // split on "." and get the last, since this should be the extension
    let splits = diff_line.split(|&byte| byte == 46);
    if let Some(ext) = splits.last() {
        return String::from_utf8(ext.to_owned()).unwrap();
    }
    return "unknown".into();
}
