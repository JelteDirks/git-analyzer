use std::process::Command;
use structures::Commit;

const COMMIT_LINE: &[u8; 6] = &[99, 111, 109, 109, 105, 116];

mod structures {
    pub struct Commit {
        pub author: String,
        pub date: u128,
        pub hash: String,
    }

    impl Commit {
        pub fn new_from_all(author_line: &[u8], date_line: &[u8],
            hash_line:&[u8]) -> Commit {

            return Commit {
                author: String::from(""),
                date: 0,
                hash: String::from(""),
            }
        }
    }
}

fn main() {
    let mut commits: Vec<Commit> = vec!();
    let mut commit_header_found: bool = false;
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
                    println!("{:?}", String::from_utf8(line.to_vec()));
                    println!("author {:?}", String::from_utf8(lines.next().unwrap().to_vec()));
                    println!("date {:?}", String::from_utf8(lines.next().unwrap().to_vec()));
                }
            }

            println!("number of commits {:?}", commits.len());
        },
        "windows" => {
            println!("not supported yet")
        }
        _ => {
            println!("wtf is even that")
        }
    }
}
