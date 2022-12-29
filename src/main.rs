use std::process::Command;
use structures::Commit;

const COMMIT_LINE: &[u8; 6] = &[99, 111, 109, 109, 105, 116];

mod structures {
    #[derive(Debug)]
    pub struct Commit {
        pub author: String,
        pub date: u128,
        pub hash: String,
    }

    impl Commit {
        pub fn new_from_all(
            hash_line:&[u8],
            author_line: &[u8],
            date_line: &[u8]) -> Commit {

            // split hash line into commit and hash
            let (_ , hash): (_ , &[u8]) = hash_line.split_at(7);

            let email = author_line
                .rsplit(|byte| *byte == 60 as u8 || *byte == 62 as u8)
                .take(2)
                .last()
                .unwrap();

            return Commit {
                author: String::from_utf8(email.to_vec()).unwrap(),
                date: 0,
                hash: String::from_utf8(hash.to_vec()).unwrap(),
            }
        }
    }
}

fn main() {
    let mut commits: Vec<Commit> = vec!();
    match std::env::consts::FAMILY {
        "unix" => {

            let output = Command::new("sh")
                .args(["-c", "git log -p"])
                .output()
                .expect("nuts");

            let mut lines = output.stdout.as_slice()
                .split(|&byte: &u8| byte == 10 as u8)
                .peekable();

            while lines.peek() != None {
                let line = lines.next().unwrap();
                let commit_text = line.get(0..6).unwrap_or(&[]);
                if commit_text == COMMIT_LINE {
                    commits.push(Commit::new_from_all(line,
                        lines.next().unwrap(),
                        lines.next().unwrap()));
                }
            }

            println!("number of commits {:?}", commits.len());
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
