use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub enum FilterType {
    Include,
    Exclude,
    Pass,
}

#[derive(Debug)]
pub struct Settings {
    pub extensions: HashMap<String, bool>,
    pub filter: FilterType,
    pub base: PathBuf,
    pub depth: u32,
    pub author: Option<String>,
}

impl Settings {
    pub fn from_args(args: crate::cli::args::Args) -> Settings {
        let mut extensions: HashMap<String, bool> = HashMap::new();
        let mut filter = FilterType::Pass;

        if args.include.is_some() {
            filter = FilterType::Include;
            args.include
                .unwrap()
                .as_bytes()
                .split(|byte| *byte == 32)
                .for_each(|bytes: &[u8]| {
                    extensions.insert(String::from_utf8(bytes.to_owned()).unwrap(), true);
                });
        } else if args.exclude.is_some() {
            filter = FilterType::Exclude;
            args.exclude
                .unwrap()
                .as_bytes()
                .split(|byte| *byte == 32)
                .for_each(|bytes: &[u8]| {
                    extensions.insert(String::from_utf8(bytes.to_owned()).unwrap(), true);
                });
        }

        let mut author = None;

        if args.author.is_some() {
            author = Some(args.author.unwrap());
        }

        let mut base = Settings::default_base();

        if args.path.is_some() {
            base = std::fs::canonicalize(args.path.unwrap()).unwrap();
        }

        let mut depth = Settings::default_depth();

        if args.depth.is_some() {
            depth = args.depth.unwrap();
        }

        return Settings {
            extensions,
            filter,
            author,
            base,
            depth,
        };
    }

    fn default_base() -> PathBuf {
        return std::env::current_dir().expect("could not get current dir");
    }

    fn default_depth() -> u32 {
        return 0;
    }
}
