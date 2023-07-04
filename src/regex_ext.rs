
use regex::Regex;

pub fn remove_regex_matches(string: &mut String, pattern: &str) {
	replace_regex_matches(string, pattern, "");
}

pub fn replace_regex_matches(string: &mut String, pattern: &str, replace: &str) {
	let pattern = Regex::new(pattern).unwrap();
	for cap in pattern.captures_iter(&string.clone()) {
			*string = string.replace(cap.get(0).unwrap().as_str(), replace);
	}
}
