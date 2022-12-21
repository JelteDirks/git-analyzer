use std::process::Command;

mod structures {
    pub struct Commit {
        pub author: String,
        pub date: u128,
    }

    impl Commit {
        fn new() -> Commit {
            return Commit {
                author: String::from(""),
                date: 0
            }
        }
    }
}

fn main() {
    match std::env::consts::FAMILY {
        "unix" => {
            println!("unix style");
            let output = Command::new("sh")
                .args(["-c", "git log -p"])
                .output()
                .expect("nuts");
            println!("number of bytes in the output: {}", output.stdout.len());
            for line in output.stdout.as_slice().split(|&byte: &u8| byte == 10 as u8) {
                println!("{:?}", line);
                let x = String::from(std::str::from_utf8(line).unwrap());
                let commit: structures::Commit;
                if let Some(substr) = x.get(0..6) {
                    println!("{}", substr);
                } else {
                    println!("too short")
                }
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
