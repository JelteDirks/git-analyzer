use std::{
    collections::HashMap,
    path:: PathBuf,
};

pub struct Settings {
    pub extension: HashMap<String, bool>,
    pub apply_filter: bool,
    pub path: PathBuf,
    pub depth: u32,
    pub command: String,
}

impl Settings {
    fn from_args(args: crate::cli::args::Args) -> Settings {
        todo!();
    }

    fn default_path() -> PathBuf {
        todo!();
    }

    fn default_depth() -> u32 {
        return 0;
    }

    fn default_command() -> String {
        return "git log -p".to_string();
    }
}
