#[derive(Debug)]
pub struct Commit {
    pub author: String,
    pub hash: String,
    pub date_unix: u32,
}

impl Commit {

    pub fn new_from_preformat(line: &[u8]) -> Commit {

        let mut chunks = line.split(|&byte| byte == 0x20);

        println!("{:?}", chunks.next().unwrap());
        println!("{:?}", chunks.next().unwrap());
        println!("{:?}", chunks.next().unwrap());

        return Commit {
            author: String::from(""),
            hash: String::from(""),
            date_unix: 30,
        }
    }

}
