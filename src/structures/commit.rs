#[derive(Debug)]
pub struct Commit {
    pub author: String,
    pub hash: String,
    pub date_unix: u32,
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

        // create a string from it and parse into u32
        let date_unix: u32 = String::from_utf8(date_slice.to_vec())
            .unwrap()
            .parse()
            .unwrap();

        return Commit {
            author: String::from_utf8(email.to_vec()).unwrap(),
            hash: String::from_utf8(hash.to_vec()).unwrap(),
            date_unix,
        }
    }
}
