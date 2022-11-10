use std::process::Command;
use std::env;
use std::path::Path;
use std::str;
use std::io::{BufReader, BufRead};
use std::fs::File;

/*
Use 
cargo test -- --test-threads=1 

Instead of 
cargo test

As the tests don't work in parallel
*/




fn assert_same_file_contents(file_name1: String, file_name2: String) {

	// open file 1
	let path1 = Path::new(&file_name1); assert!(path1.exists());
	let mut reader1 = BufReader::new(File::open(&path1).unwrap()).lines();

	// open file 2
	let path2 = Path::new(&file_name2); assert!(path2.exists());
	let mut reader2 = BufReader::new(File::open(&path2).unwrap()).lines();

	// compare
	loop {
		let read1 = reader1.next();
		let read2 = reader2.next();

		if read1.is_none() {
			assert!(read2.is_none());
			break;
		}

		if read1.is_some() {
			assert!(read2.is_some());

			let line1 = read1.unwrap().unwrap();
			let line2 = read2.unwrap().unwrap();

			assert!(line1 == line2);
		}
	}
}




fn assert_correct_file_contents(file: String) {
	let correct_file = file.clone().replace(".h", "-correct.h");
	assert_same_file_contents(file, correct_file);
}











#[test]
fn test_basic() {

	Command::new("rm")
		//.arg("-f")
		.arg("tests/basic_test/test-functions.h")
		.output()
		.expect("failed to execute process");


	Command::new("cargo")
		.arg("run")
		.arg("tests/basic_test/test.c")
		.output()
		.expect("failed to execute process");

	//
	assert_correct_file_contents("tests/basic_test/test-functions.h".to_string());

	Command::new("rm")
		//.arg("-f")
		.arg("tests/basic_test/test-functions.h")
		.output()
		.expect("failed to execute process");

}



#[test]
fn test_empty() {
	// shouldn't create file


	Command::new("cargo")
		.arg("run")
		.arg("tests/basic_empty/empty.c")
		.output()
		.expect("failed to execute process");

	assert!(!Path::new("tests/basic_empty/empty-functions.c").exists());

}




#[test]
fn test_linux() {

	Command::new("rm")
		//.arg("-f")
		.arg("tests/linux_test/linux-functions.h")
		.output()
		.expect("failed to execute process");

	Command::new("cargo")
		.arg("run")
		.arg("tests/linux_test/linux.c")
		.output()
		.expect("failed to execute process");

	assert_correct_file_contents("tests/linux_test/linux-functions.h".to_string());

	Command::new("rm")
		//.arg("-f")
		.arg("tests/linux_test/linux-functions.h")
		.output()
		.expect("failed to execute process");
}



#[test]
fn test_make() {

	let mut cd = 	env::current_dir().unwrap();
	cd.push("tests");
	cd.push("make_test");

	assert!(cd.is_dir());
	
	env::set_current_dir(cd).unwrap();

	Command::new("make")
		.arg("clean")
		.output()
		.expect("failed to execute process");

	let output = Command::new("make")
		.arg("run")
		.output()
		.expect("failed to execute process");


	assert!( Path::new("list-functions.h").exists());
	assert!(!Path::new("main-functions.h").exists());
	assert!(str::from_utf8(&output.stdout).unwrap().ends_with("01234")); // don't need to test functions file

	Command::new("make")
		.arg("clean")
		.output()
		.expect("failed to execute process");


}




