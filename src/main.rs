mod structures;
mod cli;

use crate::cli::args::Args;
use std::io::{BufRead, BufReader};
use clap::Parser;

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
    let lines: Option<BufReader<String>> = None;

    if args.stdin {
        let stdin = std::io::stdin().lock();
    } else if args.path.is_some() {
        // do a git log -p in that path
        // analyze the results of that log
    }
}


fn find_extension_from_diff(diff_line: &[u8]) -> String {
    // split on "." and get the last, since this should be the extension
    let splits = diff_line.split(|&byte| byte == 46);
    if let Some(ext) = splits.last() {
        return String::from_utf8(ext.to_owned()).unwrap();
    }
    return "unknown".into();
}
