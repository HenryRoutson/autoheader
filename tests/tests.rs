use std::process::Command;
use std::env;
use std::{thread, time};
use std::path::Path;
use std::str;

/*
Use 
cargo test -- --test-threads=1 

Instead of 
cargo test

As the tests don't work in parallel
*/



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

	thread::sleep(time::Duration::from_secs(1)); // so you can see file be added and removed

	assert!(Path::new("tests/basic_test/test-functions.h").exists());

	Command::new("rm")
		//.arg("-f")
		.arg("tests/basic_test/test-functions.h")
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

	thread::sleep(time::Duration::from_secs(1)); // so you can see file be added and removed
	
	assert!( Path::new("list-functions.h").exists());
	assert!(!Path::new("main-functions.h").exists());
	assert!(str::from_utf8(&output.stdout).unwrap().ends_with("01234"));

	Command::new("make")
		.arg("clean")
		.output()
		.expect("failed to execute process");


}
