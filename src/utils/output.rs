use std::{collections::HashMap, io::Write};

use crate::{cli::args::Args, structures::analytics::Analytic};

pub fn produce_output(analytics_list: Vec<Analytic>, args: &Args) {
    let mut analytics_collection: HashMap<String, Analytic> = HashMap::new();

    for a in analytics_list {
        let key = a.extension.as_ref().unwrap();
        analytics_collection
            .entry(key.into())
            .and_modify(|existing| {
                existing.additions += a.additions;
                existing.deletions += a.deletions;
            })
            .or_insert(a);
    }

    let mut stdout = std::io::stdout();
    let mut extension_list: Option<Vec<&[u8]>> = None;
    if args.filter_extension.is_some() {
        extension_list = Some(
            args.filter_extension
                .as_ref()
                .unwrap()
                .as_bytes()
                .split(|&byte| byte == 32)
                .collect(),
        );
    }

    for a in analytics_collection.iter() {
        let (extension, analytic) = a;
        if extension_list.is_some() {
            if is_excluded_extension(&extension, &extension_list.as_ref().unwrap()) {
                continue;
            }
        }
        stdout
            .write(format!("For {} files\n", extension).as_bytes())
            .unwrap();
        stdout
            .write(format!("\t{} additions\n", analytic.additions).as_bytes())
            .unwrap();
        stdout
            .write(format!("\t{} deletions\n", analytic.deletions).as_bytes())
            .unwrap();
    }
}

fn is_excluded_extension(extension: &str, extension_list: &Vec<&[u8]>) -> bool {
    return extension_list.contains(&extension.as_bytes());
}
