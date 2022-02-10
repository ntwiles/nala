#[macro_export]
macro_rules! regex {
    ($pattern:literal) => {
        regex::Regex::new($pattern).unwrap()
    };
}

#[macro_export]
macro_rules! assert_regex_match {
    ($re:ident, $str:expr) => {
        if !$re.is_match($str) {
            panic!(
                "\nString does not match regex. \n  String: {0} \n  Regex: {1}",
                $str, $re,
            )
        }
    };
}

#[allow(unused_imports)]
pub use {assert_regex_match, regex};
