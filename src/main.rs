mod structures;

use crate::structures::commit::Commit;
use crate::structures::analytics::Analytic;
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Mutex, Arc};
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

fn main() {
    // 1 git log with preformatted lines
    let mut commits: Vec<Commit> = Vec::new();

    match std::env::consts::FAMILY {
        "unix" => {

            // execute git log in the current directory
            let output = Command::new("sh")
                .args(["-c", "git log --format=\"%H %ct %ae\""])
                .output()
                .expect("nuts");

            // process the output of the default logging
            create_commit_structs(output.stdout.as_slice(), &mut commits);

        },
        "windows" => {
            println!("not supported yet")
        }
        _ => {
            println!("not unix / windows")
        }
    }

    let n_commits: usize = commits.len();

    let analytics: Arc<Mutex<Vec<Analytic>>> = Arc::new(Mutex::new(
        Vec::with_capacity(n_commits)));

    let mut join_handles: Vec<JoinHandle<()>> = Vec::with_capacity(n_commits);

    for commit in commits {
        let cloned_analytics = Arc::clone(&analytics);

        let handle = std::thread::spawn(move || {
            analyze_commit(commit, cloned_analytics)
        });

        join_handles.push(handle);
    }

    for h in join_handles {
        h.join().unwrap();
    }

    println!("{}", analytics.lock().unwrap().len());

    for a in analytics.lock().unwrap().iter() {
        println!("{:?}", a);
    }
}


fn analyze_commit(commit: Commit, analytics: Arc<Mutex<Vec<Analytic>>>) {
    let program = "git show ".to_owned() + &commit.hash;

    let output = Command::new("sh")
        .args(["-c", &program])
        .output()
        .expect("git show execution did not go as planned");

    let lines = output.stdout.split(|&byte| byte == 10);

    // start searching for the diff header
    let mut state: StateMachine = StateMachine::SearchingChanges;
    let mut map: HashMap<String, Analytic> = HashMap::new();
    let mut current_analytic: Option<Analytic> = None;

    for line in lines {

        // skip whitespace
        if line.len() < 2 {
            continue;
        }

        match state {
            StateMachine::SearchingPlusFile => {
                let min_comparison_slice = line.get(0..4);
                if min_comparison_slice.is_some() {
                    if min_comparison_slice.unwrap() == &PLUS_LINE {
                        // if the plus is found, start gathering the changes
                        state = StateMachine::SearchingChanges;
                    }
                }
            },

            StateMachine::SearchingMinFile => {
                let min_comparison_slice = line.get(0..4);
                if min_comparison_slice.is_some() {
                    if min_comparison_slice.unwrap() == &MIN_LINE {
                        // if the min is found, start searching for the plus
                        state = StateMachine::SearchingPlusFile;
                    }
                }
            },

            StateMachine::SearchingChanges => {
                let diff_comparison_slice = line.get(0..10);
                if diff_comparison_slice.is_some() {
                    if diff_comparison_slice.unwrap() == &DIFF_LINE {
                        // if diff header is found, start searchin for the min
                        state = StateMachine::SearchingMinFile;
                        if current_analytic.is_some() {
                            analytics.lock().unwrap()
                                .push(current_analytic.unwrap());
                        }
                        current_analytic = Some(Analytic::default());
                        current_analytic.as_mut().unwrap().extension = Some(
                            String::from_utf8(line.to_owned()).unwrap()
                        );
                        current_analytic.as_mut().unwrap().hash = Some(
                            String::from(&commit.hash)
                        );
                    }
                }
                let first_byte = line.get(0..1);
                if first_byte.is_some() {
                    if first_byte.unwrap() == &[43] {
                        // analyze as addition
                        current_analytic.as_mut().unwrap().additions += 1;
                    } else if first_byte.unwrap() == &[45] {
                        // analyze as deletion
                        current_analytic.as_mut().unwrap().deletions += 1;
                    }
                }
            },
        }
    }
}

enum StateMachine {
    SearchingMinFile,
    SearchingPlusFile,
    SearchingChanges,
}


fn create_commit_structs(hash_list: &[u8], commit_vec: &mut Vec<Commit>) {
    for line in hash_list.split(|&byte| byte == 10) {
        if line.len() < 42 { // commit hash is at least 40 characters
            // too short to be a valid commit to be investigated
            continue;
        }
        // dirty pushing, should be properly checked
        commit_vec.push(Commit::new_from_preformat(line));
    }
}
