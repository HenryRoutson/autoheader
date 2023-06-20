use std::process::Command;
use std::env;
use std::str;
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub mod test_utils;
use std::path::PathBuf;
use std::fs;

/*
Use 
cargo test -- --test-threads=1 
cargo test -- --test-threads=1 

Instead of 
cargo test

As the tests don't work in parallel
(use twice to check file removes)
*/



fn check_dir(dir: &PathBuf) {
	assert!(!dir.to_string_lossy().contains(" "), "Error - contains space : {:#?}", dir.to_str());
	assert!(dir.is_dir(), "{} : {}", dir.to_str().get_or_insert("no valid string"), fs::metadata(dir).unwrap_err());
}



#[test]
fn test_basic() {

	let mut dir = 	env::current_dir().unwrap();

	check_dir(&dir);

	dir.push("tests");
	dir.push("basic_test");

	check_dir(&dir);
	
	env::set_current_dir(&dir).unwrap();

	Command::new("rm").arg("test.h").output().expect("failed to execute process");
	Command::new("cargo").arg("run").output().expect("failed to execute process");

	test_utils::assert_correct_file_contents("test.h".to_string());

	Command::new("rm").arg("test.h").output().expect("failed to execute process");

	env::set_current_dir(&dir.parent().unwrap().parent().unwrap()).unwrap();
}


#[test]
fn test_convert() {
	
	let mut dir = env::current_dir().unwrap();

	check_dir(&dir);

	dir.push("tests");
	dir.push("convert_test1");

	check_dir(&dir);

	env::set_current_dir(&dir).unwrap();

	Command::new("mkdir").arg("output").output().expect("failed to execute process");
	Command::new("cp").arg("convert.c").arg("output/convert.c").output().expect("failed to execute process");
	Command::new("cp").arg("convert.h").arg("output/convert.h").output().expect("failed to execute process");

	dir.push("output");
	env::set_current_dir(&dir).unwrap();

	Command::new("cargo").arg("run").arg("setup").output().expect("failed to execute process");

	assert!( Path::new("convert-defs.h").exists());
	assert!( Path::new("convert.c").exists());
	assert!(!Path::new("convert.h").exists());

	// assert contents
	let mut defs_content = String::new();
	let mut c_content = String::new();

	File::open("convert-defs.h").unwrap().read_to_string(&mut defs_content).unwrap();
	File::open("convert.c").unwrap().read_to_string(&mut c_content).unwrap();

	assert!(defs_content == "");
	println!("{}", c_content);
	assert!(c_content.contains("// public"));
 
	Command::new("rm").arg("convert.c").output().expect("failed to execute process");
	Command::new("rm").arg("convert-defs.h").output().expect("failed to execute process");

	env::set_current_dir(&dir.parent().unwrap().parent().unwrap().parent().unwrap()).unwrap();
}



#[test]
fn test_empty() {

	let mut dir = env::current_dir().unwrap();

	check_dir(&dir);

	dir.push("tests");
	dir.push("empty_test");

	check_dir(&dir);

	env::set_current_dir(&dir).unwrap();

	Command::new("cargo").arg("run").output().expect("failed to execute process");

	assert!(!Path::new("empty.h").exists());

	env::set_current_dir(&dir.parent().unwrap().parent().unwrap()).unwrap();
}


#[test]
fn test_flow() {

	Command::new("make").arg("clean").output().expect("failed to execute process");

	let mut dir = env::current_dir().unwrap();

	check_dir(&dir);

	dir.push("tests");
	dir.push("flow_solver_test");

	check_dir(&dir);

	env::set_current_dir(&dir).unwrap();

	Command::new("make").output().expect("failed to execute process");

	assert!( Path::new("src/engine.h").exists());
	assert!( Path::new("src/extensions.h").exists());
	assert!(!Path::new("src/flow_solver.h").exists());
	assert!( Path::new("src/node.h").exists());
	assert!( Path::new("src/queues.h").exists());
	assert!( Path::new("src/search.h").exists());
	assert!( Path::new("src/utils.h").exists());

	Command::new("make").arg("clean").output().expect("failed to execute process");
	env::set_current_dir(&dir.parent().unwrap().parent().unwrap()).unwrap();

}

#[test]
fn test_linux() {

	let mut dir = env::current_dir().unwrap();
	dir.push("tests");
	dir.push("linux_test");

	check_dir(&dir);

	env::set_current_dir(&dir).unwrap();

	Command::new("rm").arg("linux.h").output().expect("failed to execute process");
	Command::new("cargo").arg("run").output().expect("failed to execute process");

	test_utils::assert_correct_file_contents("linux.h".to_string());

	Command::new("rm").arg("linux.h").output().expect("failed to execute process");

	env::set_current_dir(&dir.parent().unwrap().parent().unwrap()).unwrap();
}



#[test]
fn test_make() {

	let mut dir = env::current_dir().unwrap();
	dir.push("tests");
	dir.push("make_test");

	check_dir(&dir);
	
	env::set_current_dir(&dir).unwrap();

	Command::new("make").arg("clean").output().expect("failed to execute process");
	let output = Command::new("make").arg("run").output().expect("failed to execute process");

	assert!( Path::new("list.h").exists());
	assert!(!Path::new("main.h").exists());

	let output_str = str::from_utf8(&output.stdout).unwrap();
	assert!(output_str.ends_with("01234"), "OUTPUT \n{output_str}\n\n\n"); // don't need to test functions file

	Command::new("make").arg("clean").output().expect("failed to execute process");

	env::set_current_dir(&dir.parent().unwrap().parent().unwrap()).unwrap();
}


