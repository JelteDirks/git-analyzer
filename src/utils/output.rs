use std::{collections::HashMap, io::Write};

use crate::{cli::args::Args, structures::analytics::Analytic};

use super::settings::Settings;

pub fn produce_output(analytics_list: Vec<Analytic>) {
    let mut analytics_collection: HashMap<String, Analytic> = HashMap::new();
    let mut stdout_handle = std::io::BufWriter::new(std::io::stdout());

    for a in analytics_list {
        let key = a.extension.as_ref();

        if key.is_none() {
            continue;
        }

        analytics_collection
            .entry(key.unwrap().into())
            .and_modify(|existing| {
                existing.additions += a.additions;
                existing.deletions += a.deletions;
            })
            .or_insert(a);
    }

    for a in analytics_collection.iter() {
        let (extension, analytic) = a;

        write!(stdout_handle, "for {} files\n", extension).unwrap();
        write!(stdout_handle, "++ {} additions\n", analytic.additions).unwrap();
        write!(stdout_handle, "-- {} deletions\n", analytic.deletions).unwrap();
    }

    stdout_handle.flush().unwrap();
}
