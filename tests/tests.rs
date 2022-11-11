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

	let mut dir = 	env::current_dir().unwrap();
	dir.push("tests");
	dir.push("basic_test");

	assert!(dir.is_dir(), "Dir {:#?}", dir.to_str());
	env::set_current_dir(&dir).unwrap();

	Command::new("rm").arg("test-functions.h").output().expect("failed to execute process");
	Command::new("cargo").arg("run").output().expect("failed to execute process");

	assert_correct_file_contents("test-functions.h".to_string());

	Command::new("rm").arg("test-functions.h").output().expect("failed to execute process");

	env::set_current_dir(&dir.parent().unwrap().parent().unwrap()).unwrap();
}



#[test]
fn test_empty() {

	let mut dir = 	env::current_dir().unwrap();
	dir.push("tests");
	dir.push("empty_test");

	assert!(dir.is_dir(), "Dir {:#?}", dir.to_str());
	env::set_current_dir(&dir).unwrap();

	Command::new("cargo").arg("run").output().expect("failed to execute process");

	assert!(!Path::new("empty-functions.c").exists());

	env::set_current_dir(&dir.parent().unwrap().parent().unwrap()).unwrap();
}


#[test]
fn test_linux() {

	let mut dir = 	env::current_dir().unwrap();
	dir.push("tests");
	dir.push("linux_test");

	assert!(dir.is_dir(), "Dir {:#?}", dir.to_str());
	env::set_current_dir(&dir).unwrap();

	Command::new("rm").arg("linux-functions.h").output().expect("failed to execute process");
	Command::new("cargo").arg("run").output().expect("failed to execute process");

	assert_correct_file_contents("linux-functions.h".to_string());

	Command::new("rm").arg("linux-functions.h").output().expect("failed to execute process");

	env::set_current_dir(&dir.parent().unwrap().parent().unwrap()).unwrap();
}



#[test]
fn test_make() {

	let mut dir = 	env::current_dir().unwrap();
	dir.push("tests");
	dir.push("make_test");

	assert!(dir.is_dir(), "Dir {:#?}", dir.to_str());
	env::set_current_dir(&dir).unwrap();

	Command::new("make").arg("clean").output().expect("failed to execute process");
	let output = Command::new("make").arg("run").output().expect("failed to execute process");

	assert!( Path::new("list-functions.h").exists());
	assert!(!Path::new("main-functions.h").exists());
	assert!(str::from_utf8(&output.stdout).unwrap().ends_with("01234")); // don't need to test functions file

	Command::new("make").arg("clean").output().expect("failed to execute process");

	env::set_current_dir(&dir.parent().unwrap().parent().unwrap()).unwrap();
}


