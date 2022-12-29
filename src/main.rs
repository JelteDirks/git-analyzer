mod structures;

use std::process::Command;
use structures::commit::Commit;

const COMMIT_LINE: &[u8; 6] = &[99, 111, 109, 109, 105, 116];

fn main() {
    let mut commits: Vec<Commit> = vec!();
    match std::env::consts::FAMILY {
        "unix" => {

            // execute git log in the current directory
            let output = Command::new("sh")
                .args(["-c", "git log -p"])
                .output()
                .expect("nuts");

            // create an iterator over each individual line (as bytes)
            let mut lines = output.stdout.as_slice()
                .split(|&byte: &u8| byte == 10 as u8)
                .peekable();

            // iterate over the lines one by one untill there is None left
            while lines.peek() != None {
                // get the next line
                let line = lines.next().unwrap();
                // get the first 6 bytes as a slice or an empty slice
                let commit_text = line.get(0..6).unwrap_or(&[]);
                // check if the first 6 bytes spell "commit"
                if commit_text == COMMIT_LINE {
                    // if they do, create a new commit from the next few lines
                    commits.push(Commit::new_from_all(line,
                        lines.next().unwrap(),
                        lines.next().unwrap()));
                }
            }

            for commit in commits {
                println!("{:?}", commit);
            }
        },
        "windows" => {
            println!("not supported yet")
        }
        _ => {
            println!("wtf is even that")
        }
    }
}
