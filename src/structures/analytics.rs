#[derive(Debug)]
pub struct Analytic {
    pub extension: String,
    pub additions: u32,
    pub deletions: u32,
    pub author: Option<String>,
}

impl Analytic {
    pub fn with_extension(extension: String) -> Analytic {
        return Analytic {
            author: None,
            extension,
            additions: 0,
            deletions: 0,
        };
    }
}
