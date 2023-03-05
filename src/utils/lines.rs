use std::io::{Lines, StdinLock};

use crate::{structures::analytics::Analytic, cli::args};

pub fn is_addition(line: &str) -> bool {
    if line.len() < 1 {
        return false;
    }
    if line.get(0..1).unwrap() == "+" {
        return true;
    }
    return false;
}

pub fn is_deletion(line: &str) -> bool {
    if line.len() < 1 {
        return false;
    }
    if line.get(0..1).unwrap() == "-" {
        return true;
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

enum AnalyzeState {
    DiffLine,
    Changes,
}

pub fn process_byte_slice<'a>(
    bytes: &[u8],
    analytics_list: &'a mut Vec<Analytic>,
) -> &'a mut Vec<Analytic> {
    let mut state = AnalyzeState::DiffLine;
    let mut analytic = Analytic::default();
    let byte_lines = bytes.split(|&byte| byte == 10);

    for byte_line in byte_lines {
        let line_result = std::str::from_utf8(byte_line);

        if line_result.is_err() {
            todo!("error in the line from stdin, handle it gracefully");
        }

        let line = line_result.unwrap();

        if is_diff_line(&line) {
            analytics_list.push(analytic);
            analytic = Analytic::default();
            let ext = find_extension_from_diff(&line.as_bytes());
            analytic.extension = Some(ext.into());
            continue;
        }
        if is_addition(&line) {
            analytic.additions += 1;
        } else if is_deletion(&line) {
            analytic.deletions += 1;
        }

    }
    analytics_list.push(analytic);
    return analytics_list;
}

pub fn process_stdin_lines<'a>(
    lines: Lines<StdinLock>,
    analytics_list: &'a mut Vec<Analytic>,
) -> &'a mut Vec<Analytic> {
    let mut state = AnalyzeState::DiffLine;
    let mut analytic = Analytic::default();

    for line_result in lines {
        if line_result.is_err() {
            todo!("error in the line from stdin, handle it gracefully");
        }

        let line = line_result.unwrap();

        match state {
            AnalyzeState::DiffLine => {
                if is_diff_line(&line) {
                    state = AnalyzeState::Changes;
                    let ext = find_extension_from_diff(&line.as_bytes());
                    analytic.extension = Some(ext.into());
                }
            }
            AnalyzeState::Changes => {
                if is_diff_line(&line) {
                    analytics_list.push(analytic);
                    analytic = Analytic::default();
                    let ext = find_extension_from_diff(&line.as_bytes());
                    analytic.extension = Some(ext.into());
                    // TODO: do the saving of this analytic in here and continue
                    // with the new diff to analyze
                    continue;
                }
                if is_addition(&line) {
                    analytic.additions += 1;
                } else if is_deletion(&line) {
                    analytic.deletions += 1;
                }
            }
        }
    }
    analytics_list.push(analytic);
    return analytics_list;
}
