use regex::Regex;

pub fn strip_comments(code: String) -> String {
    let single_line = Regex::new(r"//.*").unwrap();
    let multi_line = Regex::new(r"/\*.*\*/").unwrap();

    let stripped = single_line.replace_all(&code, "").into_owned();
    multi_line.replace_all(&stripped, "").into_owned()
}
