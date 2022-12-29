use std::process::Command;
use structures::Commit;

const COMMIT_LINE: &[u8; 6] = &[99, 111, 109, 109, 105, 116];

mod structures {

    use chrono::naive::NaiveDate;

    #[derive(Debug)]
    pub struct Commit {
        pub author: String,
        pub date: NaiveDate, // possibly use long format from git log
        pub hash: String,
    }

    impl Commit {
        pub fn new_from_all(
            hash_line:&[u8],
            author_line: &[u8],
            date_line: &[u8]) -> Commit {

            // split hash line into commit and hash
            let (_ , hash): (_ , &[u8]) = hash_line.split_at(7);

            // filter out email between < and >
            let email = author_line
                .rsplit(|byte| *byte == 60 as u8 || *byte == 62 as u8)
                .take(2)
                .last()
                .unwrap();

            // rettrieve the date from position
            let (_, date_slice): (_, &[u8]) = date_line
                .split_at(8);

            // create a string from it
            let date_string = String::from_utf8(date_slice.to_vec()).unwrap();

            // create a naive date using the format of default git log
            let date: NaiveDate = NaiveDate::parse_from_str(&date_string,
                "%a %b %d %H:%M:%S %Y %z") .unwrap();

            return Commit {
                author: String::from_utf8(email.to_vec()).unwrap(),
                date,
                hash: String::from_utf8(hash.to_vec()).unwrap(),
            }
        }
    }
}

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
