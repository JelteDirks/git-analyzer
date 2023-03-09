#[derive(Debug)]
pub struct Analytic {
    pub extension: Option<String>,
    pub additions: u32,
    pub deletions: u32,
}

impl Analytic {
    pub fn default() -> Analytic {
        return Analytic {
            extension: None,
            additions: 0,
            deletions: 0,
        };
    }

    pub fn from_ref(a: &Analytic) -> Analytic {
        let mut default = Analytic::default();

        default.extension = match &a.extension {
            Some(s) => Some(String::from(s)),
            None => None,
        };

        default.additions = a.additions;
        default.deletions = a.deletions;

        return default;
    }
}
