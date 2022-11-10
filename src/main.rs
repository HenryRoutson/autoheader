
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


/*
TODO

deploy 
*/

use std::fs::File;
use std::env;
use std::path::Path;
use std::io::{Read, Write};
use std::os::unix::ffi::OsStrExt;


fn main() {

    let args_error = 
    "\n\nfile not provided\n
        please use the below format

        autoheader <path from current directory to c file>
        autoheader linkedlist.c\n\n";
    
    let write_error = "Error writing to file";

    let public_tag = "// public";

    // process cli args

    let mut args = env::args();
    let c_file_string = &args.nth(1).expect(args_error); 
    assert!(c_file_string.ends_with(".c"), "file does not have .c extension");

    // open .c file contents
    let c_file_path = Path::new(&c_file_string); 
    assert!(c_file_path.exists(), "c file does not exist");

    let mut c_file_content = String::new();
    File::open(c_file_path).expect("Cannot open file").read_to_string(&mut c_file_content).unwrap();
    if !c_file_content.contains(public_tag) {
        println!("This file didn't contain any public tags,\na functions file was not created");
        return;
    }

    // create h file
    let h_file_string = c_file_string[..c_file_string.len() - 2].to_string() + &"-functions.h".to_string();  
    let h_file_path = Path::new(&h_file_string);
    let mut h_file = File::create(h_file_path).expect("could not create header file");

    // #include "****-structs.h" in functions.h for defined types
    h_file.write(b"#include \""                             ).expect(write_error);
    h_file.write(c_file_path.file_stem().unwrap().as_bytes()).expect(write_error);
    h_file.write( b"-structs.h\"\n\n"                       ).expect(write_error);

    for s in c_file_content.split(public_tag).skip(1) {
        let function_prototype = s[..s.find('{').unwrap()-1].trim().as_bytes();
        h_file.write(&function_prototype).expect(write_error);
        h_file.write(b";\n"             ).expect(write_error);
    } 

    println!("Done");
}
