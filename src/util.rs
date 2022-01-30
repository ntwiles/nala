// TODO: Ignoring several warnings here that don't make sense, look into this.
#[allow(unused_macros)]
macro_rules! regex {
    ($pattern:literal) => {
        regex::Regex::new($pattern).unwrap()
    };
}

#[allow(unused_macros)]
macro_rules! assert_regex_match {
    ($re:ident, $str:expr) => {
        if !$re.is_match($str) {
            panic!(
                "\nString does not match regex. \n  String: {0} \n  Regex: {1}",
                $str,
                $re.to_string(),
            )
        }
    };
}

#[allow(unused_imports)]
pub(crate) use {assert_regex_match, regex};
