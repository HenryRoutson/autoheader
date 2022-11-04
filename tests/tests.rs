use std::process::Command;
use std::env;


// sometimes doesn't create new file


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

	Command::new("make")
		.arg("run")
		.output()
		.expect("failed to execute process");

	
	Command::new("make")
		.arg("clean")
		.output()
		.expect("failed to execute process");

}