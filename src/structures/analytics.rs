#[derive(Debug)]
pub struct Analytic {
    language: Language,
    additions: u32,
    deletions: u32,
}

impl Analytic {
    pub fn with_language(language: Language) -> Analytic {
        return Analytic {
            language,
            additions: 0,
            deletions: 0,
        };
    }
}

#[derive(Debug)]
pub enum Language {
    C,
    Go,
    JavaScript,
    TypeScript,
    Rust,
}
