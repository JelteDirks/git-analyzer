mod structures;

use crate::structures::commit::Commit;
use crate::structures::analytics::Analytic;
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

    println!("{:?}", output);
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
