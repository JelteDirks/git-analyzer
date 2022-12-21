use std::process::Command;
use structures::Commit;

mod structures {
    pub struct Commit {
        pub author: String,
        pub date: u128,
        pub hash: Vec<u8>,
    }

    impl Commit {
        pub fn from_hashline(hasline: &str) -> Commit {
            return Commit {
                author: String::from(""),
                date: 0,
                hash: vec!(),
            }
        }
    }
}

fn main() {
    let mut commits: Vec<Commit> = vec!();
    match std::env::consts::FAMILY {
        "unix" => {
            println!("unix style");
            let output = Command::new("sh")
                .args(["-c", "git log -p"])
                .output()
                .expect("nuts");
            println!("number of bytes in the output: {}", output.stdout.len());
            for line in output.stdout.as_slice().split(|&byte: &u8| byte == 10 as u8) {
                let x = String::from(std::str::from_utf8(line).unwrap());
                if let Some(substr) = x.get(0..6) {
                    if substr == "commit" {
                        commits.push(Commit::from_hashline(substr));
                    }
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
