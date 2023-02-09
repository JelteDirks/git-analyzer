mod structures;
mod cli;

use crate::structures::analytics::Analytic;
use crate::structures::commit::Commit;
use clap::Parser;
use std::collections::HashMap;
use std::io::{stdin, BufRead};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

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

// byte slice that spells: "diff --git"
const DIFF_LINE: [u8; 10] = [100, 105, 102, 102, 32, 45, 45, 103, 105, 116];

// byte slice for "--- "
const MIN_LINE: [u8; 4] = [45, 45, 45, 32];

// byte slice for "+++ "
const PLUS_LINE: [u8; 4] = [43, 43, 43, 32];


#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short = 'i', long)]
    stdin: bool,

    #[arg(short = 'p', long)]
    path: Option<String>,
}

fn main() {

    let args = Args::parse();

    if args.stdin {
        let stdin = std::io::stdin().lock();
        let lines = stdin.lines();

        for line in lines {
            println!("{:?}", line);
        }
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
