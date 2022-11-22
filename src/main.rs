
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

////////////////////////////////////////////////

static PUBLIC_TAG: &str = "// public";
static H_FILE_EXPLAINATION: &[u8;160] = b"\
// This file was automatically created,\n\
// any defitions, including typedefs, structs, extern or #define\n\
// have been moved to a -defs.h file of the same name\n\n";

////////////////////////////////////////////////


/*
TODO

turn into functions

convert automatically

    remove inbetween {}
    (static\s*)?(unsigned\s*)?(\s+)\w+(\s+|(\s?\*+\s*)+)\w+\s*\(
            struct
            long 
            short
            Need to remove comments 
            Split into types and names string

    detect function prototypes
    add public tag
    remove functions from .h
    rename to -defs.h

Avoid needing to store executable, use crates.io?

Put in psuedo email for feedback

*/

use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::fs;
use colored::*;


fn main() {

    print!("\n\n");
    let paths = fs::read_dir("./").expect("Error finding current directory");
    for path in paths {

        let c_file_path = path.expect("Error path does not exist").path();
        let c_file_string = c_file_path.to_str().expect("path to string error");

        create_header_file(c_file_string);
    }
    println!("\nDone\n\n");
}

fn create_header_file(c_file_string: &str) {

    // CODE
    if !c_file_string.ends_with(".c") { println!("{}", format!(" {} : {} ", "doesn't end with .c file extension", c_file_string).on_truecolor(247, 103, 87)); return; } // red

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
        let _ = File::create(defs_path).unwrap();
        println!(" {} : {} ", "defs file was created", defs_string);
    }

    // create h file
    let write_error = "Error writing to file";
    let h_file_string = c_file_string.replace(".c", ".h");  
    let h_file_path = Path::new(&h_file_string);
    let mut h_file = File::create(h_file_path).expect("could not create header file");
    assert!(h_file_path.exists());
    println!("{}", format!(" {} : {} ", "functions prototype file was created", h_file_string).on_truecolor(135, 245, 166)); // green

    h_file.write(H_FILE_EXPLAINATION).expect(write_error); 

    // #include "****-defs.h" in functions.h for defined types
    h_file.write(b"#include \""                             ).expect(write_error);
    h_file.write(c_file_path.file_stem().expect("Error: no file stem").as_bytes()).expect(write_error);
    h_file.write( b"-defs.h\"\n\n"                       ).expect(write_error);

    for s in c_file_content.split(PUBLIC_TAG).skip(1) {
        let function_prototype = s[..s.find('{').expect("{ not found after // public")-1].trim().as_bytes();
        h_file.write(&function_prototype).expect(write_error);
        h_file.write(b";\n"             ).expect(write_error);
    } 
}


