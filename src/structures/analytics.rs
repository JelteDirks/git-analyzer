#[derive(Debug)]
pub struct Analytic {
    pub extension: Option<String>,
    pub additions: u32,
    pub deletions: u32,
    pub author: Option<String>,
    pub hash: Option<String>,
    pub min_line: Option<String>,
    pub plus_line: Option<String>,
}

impl Analytic {
    pub fn default() -> Analytic {
        return Analytic {
            author: None,
            extension: None,
            additions: 0,
            deletions: 0,
            hash: None,
            min_line: None,
            plus_line: None,
        };
    }
}
