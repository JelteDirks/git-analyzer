use chrono::naive::NaiveDate;

#[derive(Debug)]
pub struct Commit {
    pub author: String,
    pub date: NaiveDate, // possibly use long format from git log
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

        // create a string from it
        let date_string = String::from_utf8(date_slice.to_vec()).unwrap();

        // create a naive date using the format of default git log
        let date: NaiveDate = NaiveDate::parse_from_str(&date_string,
            "%a %b %d %H:%M:%S %Y %z") .unwrap();

        return Commit {
            author: String::from_utf8(email.to_vec()).unwrap(),
            date,
            hash: String::from_utf8(hash.to_vec()).unwrap(),
            date_unix: 40,
        }
    }
}
