
/*///////////////////////////////////////////////

    Automate creating and updating header files in c
    by creating a header file containing all functions tagged with the public tag in the c file

    ------------------
    .c
    // public
    int function() {
        ...;
    }
    ------------------
    .h
    int function();
    ------------------

*////////////////////////////////////////////////

// test with 
// cargo test -- --test-threads=1 

// visualise structure with
// cargo modules generate tree

////////////////////////////////////////////////

static PUBLIC_TAG: &str = "// public\n";
static H_FILE_EXPLAINATION: &[u8;160] = b"\
// This file was automatically created,\n\
// any defitions, including typedefs, structs, extern or #define\n\
// have been moved to a -defs.h file of the same name\n\n";
static WRITE_ERROR: &str = "Error writing to file";

////////////////////////////////////////////////


/*
TODO

Avoid needing to store executable, use crates.io?

Put in psuedo email for feedback

multithreading

*/

use std::fs::File;
use std::ops::Add;
use std::path::Path;
use std::io::{Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::fs;
use colored::*;
use std::env;
use std::process::Command;


pub mod regex_ext;
pub mod c;


fn main() {


    let mut args =  env::args().skip(1);

    let mut run_setup: bool = false;
    if args.len() >= 1 {
        run_setup = args.next().unwrap().to_lowercase() == "setup";
    }

    print!("\n\n");
    let paths = fs::read_dir("./").expect("Error finding current directory");
    for path in paths {

        let file_path = path.expect("Error path does not exist").path();
        let file_string = file_path.to_str().expect("path to string error");

        if run_setup {
            setup(file_string);
        } else {
            create_h(file_string);
        }
    }
    println!("\nDone\n\n");
    

}



fn setup(file_string: &str) {

    println!("\n\n\n\n Running  Setup \n  {} \n", file_string);

    if file_string.ends_with("-defs.h") { println!("-defs file already exists"); return; }

    // check if c or h file
    let ext = &file_string[file_string.len()-2..file_string.len()]; // extension
    if ext != ".c" && ext != ".h" { println!("file didn't contain a c or h extension"); return; }

    // check if defs file alreadys exists and setup is already done
    let file_string_no_ext = &file_string[0..file_string.len()-2];
    let defs_path = file_string_no_ext.to_string().add("-defs.h"); // remove extension and add
    let defs_path = Path::new(&defs_path);
    if defs_path.exists() {  println!("-defs file already exists"); return; } 

    // get functions from h file
    let h_file_path = file_string_no_ext.to_string().add(".h");
    if !Path::new(&h_file_path).exists() { println!("h file didn't exist"); return; }
    let functions = c::isolate_functions(&h_file_path);

    // read files to string
    let c_file_path = file_string_no_ext.to_string().add(".c");
    let mut c_file = File::open(&c_file_path).expect("c file didn't exist");
    let mut h_file = File::open(&h_file_path).expect("h file didn't exist");
    
    let mut c_file_content = String::new();
    c_file.read_to_string(& mut c_file_content).unwrap();

    let mut h_file_content = String::new();
    h_file.read_to_string(& mut h_file_content).unwrap();

    for prototype in functions.split("\n") {
        
        let function  = &prototype[..prototype.len()-1]; // remove semicolon   int x(); -> int x()
        println!("{}", function);

        // add public tags to c file
        c_file_content = c_file_content.replace(&function, &(PUBLIC_TAG.to_string() + &function));

        // remove function prototypes from h file
        h_file_content = h_file_content.replace(prototype, "");
    }

    // remove old c and h files
    Command::new("rm").arg(&c_file_path).output().expect("failed to execute process");
    Command::new("rm").arg(&h_file_path).output().expect("failed to execute process");

    // write back c and h file
    File::create(&c_file_path).unwrap().write(c_file_content.as_bytes()).unwrap();
    File::create(&h_file_path).unwrap().write(h_file_content.as_bytes()).unwrap();

    // rename h file to -defs.h
    Command::new("mv").arg(&h_file_path).arg(&defs_path).output().expect("failed to execute process");

}


fn create_h(file_string: &str) { // use

    if !file_string.ends_with(".c") { println!("{}", format!(" {} : {} ", "doesn't end with .c file extension", file_string).on_truecolor(247, 103, 87)); return; } // red
    let c_file_string = file_string;

    // open .c file contents++
    let c_file_path = Path::new(&c_file_string); 
    assert!(c_file_path.exists(), "c file does not exist");

    let mut c_file_content = String::new();
    File::open(c_file_path).expect("Cannot open file").read_to_string(&mut c_file_content).expect("Error reading file contetnts to string");
    if !c_file_content.contains(PUBLIC_TAG) {
        println!("{}",format!(" {} : {} ", "no public tags, a function prototype file wasn't created", c_file_string).on_truecolor(235, 177, 52)); // orange
        return;
    }

    // create defs file if none
    let defs_string = c_file_string.replace(".c","-defs.h");
    let defs_path = Path::new(&defs_string);
    if !defs_path.exists() {
        File::create(defs_path).expect("could not create defs file");
        println!(" {} : {} ", "defs file was created", defs_string);
    }

    // create h file
    let h_file_string = c_file_string.replace(".c", ".h");  
    let h_file_path = Path::new(&h_file_string);
    let mut h_file = File::create(h_file_path).expect("could not create header file");
    println!("{}", format!(" {} : {} ", "functions prototype file was created", h_file_string).on_truecolor(135, 245, 166)); // green

    h_file.write(H_FILE_EXPLAINATION).expect(WRITE_ERROR); 

    // #include "****-defs.h" in functions.h for defined types
    h_file.write(b"#include \""                                                       ).expect(WRITE_ERROR);
    h_file.write(c_file_path.file_stem().expect("Error: no file stem").as_bytes()).expect(WRITE_ERROR);
    h_file.write( b"-defs.h\"\n\n"                                                    ).expect(WRITE_ERROR);

    for s in c_file_content.split(PUBLIC_TAG).skip(1) {
        let function_prototype = s[..s.find('{').expect("{ not found after // public")-1].trim().as_bytes();
        h_file.write(&function_prototype).expect(WRITE_ERROR);
        h_file.write(b";\n"             ).expect(WRITE_ERROR);
    } 
}