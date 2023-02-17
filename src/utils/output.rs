use std::{collections::HashMap, io::Write};

use crate::{cli::args::Args, structures::analytics::Analytic};

pub fn produce_output(analytics_list: Vec<Analytic>, args: &Args) {
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

    let mut extension_list: Option<Vec<&[u8]>> = None;
    if args.include.is_some() || args.exclude.is_some() {
        extension_list = Some(
            args.include
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
            let included = extension_list.as_ref().unwrap().contains(&extension.as_bytes());
            if args.include.is_some() && !included {
                continue;
            } else if args.exclude.is_some() && included {
                continue;
            }
        }
        write!(stdout_handle, "for {} files\n", extension).unwrap();
        write!(stdout_handle, "++ {} additions\n", analytic.additions).unwrap();
        write!(stdout_handle, "-- {} deletions\n", analytic.deletions).unwrap();
    }

    stdout_handle.flush().unwrap();
}

