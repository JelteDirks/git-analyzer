pub fn is_addition(line: &Result<String, std::io::Error>) -> bool {
    let actual = line.as_ref().unwrap();
    if actual.len() < 1 {
        return false;
    }
    if actual.get(0..1).unwrap() == "+" {
        return true;
    }
    return false;
}

pub fn is_deletion(line: &Result<String, std::io::Error>) -> bool {
    let actual = line.as_ref().unwrap();
    if actual.len() < 1 {
        return false;
    }
    if actual.get(0..1).unwrap() == "-" {
        return true;
    }
    return false;
}

pub fn is_diff_line(line: &Result<String, std::io::Error>) -> bool {
    let actual = line.as_ref().unwrap();
    if actual.len() < 10 {
        return false;
    }
    if actual.get(0..10).unwrap() == "diff --git" {
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
