
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::regex_ext;

pub fn isolate_functions(file_string: &str) -> String {

	if !Path::new(file_string).exists() { panic!("File does not exist {}", file_string); }

	let mut file = File::open(file_string).expect("couldn't open file");
	let mut file_contents = String::new();
	file.read_to_string(&mut file_contents).unwrap();

	// remove inside brackets
	regex_ext::remove_regex_matches(&mut file_contents, r"\{[^}]*\}");

	// remove comments
	regex_ext::remove_regex_matches(&mut file_contents, r"/\*[\w\W]*?\*/"); /* */
	regex_ext::remove_regex_matches(&mut file_contents, r"//[^\n]*\n"); //

	// remove fake newlines
	file_contents = file_contents.replace("/\n", "");

	// remove duplicate brackets
	regex_ext::remove_regex_matches(&mut file_contents, r"/"); //

	// remove imports
	regex_ext::remove_regex_matches(&mut file_contents, r"#.*");

	// remove extern
	regex_ext::remove_regex_matches(&mut file_contents, r"extern[\w\[\]\*\s]+?;");

	// remove type definitions
	regex_ext::remove_regex_matches(&mut file_contents, r"typedef[^;]*;");
	regex_ext::remove_regex_matches(&mut file_contents, r"enum[^;]*;");

	// remove unneeded lines
	regex_ext::replace_regex_matches(&mut file_contents, "\n+", "\n");

	file_contents
}






#[cfg(test)]
mod tests {

	use super::isolate_functions;

	use std::env;

	#[test]
	fn test_isolate () {

		let mut path = env::current_dir().unwrap();
		path.push("tests");
		path.push("isolate_test");
		path.push("isolate.h");
		
		let mut file_contents = isolate_functions(path.to_str().unwrap());
		assert!(file_contents.lines().count() > 0);

		// println!("\n{}\n", file_contents); // manual inspect

		// remove functions

		file_contents = "\n".to_owned() + &file_contents;
		crate::regex_ext::remove_regex_matches(&mut file_contents, r"([\*\s]+\w+)+\([\w\s\*,]*\)");

		// remove other

		file_contents = file_contents.replace("\n", "");
		file_contents = file_contents.replace(";", "");
		file_contents = file_contents.replace(" ", "");

		if file_contents != "" {
			panic!("\nFile didn't isolate functions \nSTART\n{}\nEND\n", file_contents);
		}

	}
}