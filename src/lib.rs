
pub fn try_exists(path: &str) -> bool{
    let result = std::fs::metadata(path);
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn captures_into_vec(captures: regex::CaptureMatches) -> Vec<String> {
    let mut vec = Vec::new();
    for capture in captures {
        vec.push(capture.get(0).unwrap().as_str().to_string());
    }
    vec
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn test_try_exists() {
        assert_eq!(try_exists("./src/lib.rs"), true);
        assert_eq!(try_exists("./src/lib.rsx"), false);
    }

    #[test]
    fn test_captures_into_vec() {
        let re = Regex::new(r"a\(:.*?:\)").unwrap();
        let src_string = String::from("a(:a:) a(:b:) a(:c:)");
        let aliases = re.captures_iter(&src_string);
        let aliases = captures_into_vec(aliases);
        assert_eq!(aliases, vec!["a(:a:)", "a(:b:)", "a(:c:)"]);
    }
}
