#[derive(Debug)]
pub struct Commit {
    pub author: String,
    pub hash: String,
    pub date_unix: u32,
}

impl Commit {

    // convert a line from the preformatted log output to a commit structure
    pub fn new_from_preformat(line: &[u8]) -> Commit {

        // split on spaces
        let mut chunks = line.split(|&byte| byte == 0x20);

        // first character set is the hash
        let hash_slice: &[u8] = chunks.next().unwrap();
        let hash_string: &str = std::str::from_utf8(hash_slice).unwrap();

        // second character set is the long commit date
        let date_slice: &[u8] = chunks.next().unwrap();
        let date_string: &str = std::str::from_utf8(date_slice).unwrap();
        let date_long: u32 = date_string.parse().unwrap();

        // third character set is the email of the commiter
        let author_email_slice: &[u8] = chunks.next().unwrap();
        let author_email_string: &str = std::str::from_utf8(author_email_slice)
            .unwrap();

        // create and return the commit with owned strings
        return Commit {
            author: author_email_string.into(),
            hash: hash_string.into(),
            date_unix: date_long,
        }
    }

}
