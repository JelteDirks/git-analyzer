use std::io::Write;
use super::settings::{FilterType, Settings};
use crate::structures::analytics::Analytic;

pub fn is_addition(line: &str) -> bool {
    if line.len() < 1 {
        return false;
    }
    if line.get(0..1).unwrap() == "+" {
        if line.get(0..5).is_some() {
            return line.get(0..5).unwrap() != "+++ b";
        } else {
            return true;
        }
    }
    return false;
}

pub fn is_deletion(line: &str) -> bool {
    if line.len() < 1 {
        return false;
    }
    if line.get(0..1).unwrap() == "-" {
        if line.get(0..5).is_some() {
            return line.get(0..5).unwrap() != "--- a";
        } else {
            return true;
        }
    }
    return false;
}

pub fn is_diff_line(line: &str) -> bool {
    if let Some("diff --git") = line.get(0..10) {
        return true;
    }

    return false;
}

pub fn find_extension_from_diff(diff_line: &[u8]) -> String {
    // split on "." and get the last, since this should be the extension
    // improve this function since there might be files without extension
    // possibly use the entire filename
    let splits = diff_line.split(|&byte| byte == 46);
    if let Some(ext) = splits.last() {
        return String::from_utf8(ext.to_owned()).unwrap();
    }
    return "unknown".into();
}

pub fn process_byte_slice(bytes: &[u8], analytics_list: &mut Vec<Analytic>, settings: &Settings) {
    let mut analytic = Analytic::default();
    let byte_lines = bytes.split(|&byte| byte == 10);
    let mut ignore = false;

    for byte_line in byte_lines {
        let line_result = std::str::from_utf8(byte_line);

        if line_result.is_err() {
            write!(
                std::io::stderr(),
                "problem converting byte line {:?}\n",
                byte_line
            )
            .unwrap();
            continue;
        }

        let line = line_result.unwrap();

        if is_diff_line(&line) {
            let ext = find_extension_from_diff(&line.as_bytes());

            match settings.filter {
                FilterType::Pass => {}
                FilterType::Include => {
                    ignore = settings.extensions.get(&ext.to_string()).is_none();
                }
                FilterType::Exclude => {
                    ignore = *settings.extensions.get(&ext.to_string()).unwrap_or(&false);
                }
            }

            if ignore == true {
                continue;
            }

            analytics_list.push(analytic);
            analytic = Analytic::default();
            analytic.extension = Some(ext.into());
            continue;
        }

        if ignore == true {
            continue;
        }

        if is_addition(&line) {
            analytic.additions += 1;
        } else if is_deletion(&line) {
            analytic.deletions += 1;
        }
    }

    analytics_list.push(analytic);
}
