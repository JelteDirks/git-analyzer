mod structures;

use std::process::Command;
use structures::commit::Commit;


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
//

fn main() {
    let mut commits: Vec<Commit> = vec!();
    match std::env::consts::FAMILY {
        "unix" => {

            // execute git log in the current directory
            let output = Command::new("sh")
                .args(["-c", "git log --format=\"%H %ct %ae\""])
                .output()
                .expect("nuts");

            // process the output of the default logging
            process_git_hashes(output.stdout.as_slice(), &mut commits);

        },
        "windows" => {
            println!("not supported yet")
        }
        _ => {
            println!("not unix / windows")
        }
    }
}

fn process_git_hashes(hash_list: &[u8], commit_vec: &mut Vec<Commit>) {
    for line in hash_list.split(|&byte| byte == 10) {
        if line.len() < 42 { // commit hash is at least 40 characters
            // too short to be a valid commit to be investigated
            continue;
        }
        // dirty pushing, should be properly checked
        commit_vec.push(Commit::new_from_preformat(line));
    }
}
