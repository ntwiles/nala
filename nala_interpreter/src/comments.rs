use regex::Regex;

/* TODO: Both of these regex solutions are imperfect. This will match comment patterns
 * inside strings for example and strip them. Consider moving from preprocess stripping
 * comments to adding them to the grammar as recommended here:
 *
 * https://blog.ostermiller.org/finding-comments-in-source-code-using-regular-expressions/
 */
pub fn strip_comments(code: String) -> String {
    let single_line = Regex::new(r"//.*").unwrap();
    let multi_line = Regex::new(r"/\*(.|[\r\n])*?\*/").unwrap();

    let stripped = single_line.replace_all(&code, "").into_owned();
    multi_line.replace_all(&stripped, "").into_owned()
}
