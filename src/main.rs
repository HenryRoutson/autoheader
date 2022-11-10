
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

/*
TODO

assert output of basic_test
get working for multline function declarations
get working for functions not ending with {

deploy 
*/

use std::fs::File;
use std::env;
use std::path::Path;
use std::io::{BufReader, BufRead, Write};
use std::os::unix::ffi::OsStrExt;


fn main() {

    // define constants

    let public_tag = "// public";
    let command_line_input_description = 
    "
    please use the below format
        header <path from current directory to c file>
        header linkedlist.c\n
    ";

    // process cli args

    let num_args = 1;
    let mut args = env::args();
    let _cd = args.next().unwrap();
    assert!(args.len() == num_args, "\n\nWrong number of arguements provided\nprovided {} expected {}\n{}", args.len(), num_args, command_line_input_description);

    // open .c file


    let c_file_string = &args.next().expect("\n\nfile not provided\n\n"); 
    assert!(c_file_string.ends_with(".c"), "file does not have .c extension");

    let c_file_path = Path::new(&c_file_string); 
    assert!(c_file_path.exists(), "c file does not exist in the current directory");

    let c_file_buffer_reader = BufReader::new(File::open(c_file_path).expect("Cannot open file"));
    let mut c_file_lines = c_file_buffer_reader.lines();

    //

    let mut h_file = None;

    // iterate through the c file as a text file


    loop {

        let line =  c_file_lines.next();

        if line.is_none() { break; }
        if line.unwrap().unwrap().starts_with(public_tag) {

            if h_file.is_none() { // only create file if if will contain anything

                // create functions.h file
                let h_file_string = c_file_string[..c_file_string.len() - 2].to_string() + &"-functions.h".to_string();  
                let h_file_path = Path::new(&h_file_string);
                h_file = Some(File::create(h_file_path).expect("could not create header file"));

                // #include "****-structs.h" in functions.h for defined types
                h_file.as_ref().unwrap().write(b"#include \""                             ).expect("Error writing file");
                h_file.as_ref().unwrap().write(c_file_path.file_stem().unwrap().as_bytes()).expect("Error writing file");
                h_file.as_ref().unwrap().write( b"-structs.h\"\n\n"                       ).expect("Error writing file");
            }

            let function_str = c_file_lines.next().unwrap().unwrap();
            let function_prototype = function_str[..function_str.find("{") // for one line functions
                .expect("functions definition lines need to end with{")]
                .as_bytes(); 

            h_file.as_ref().unwrap().write(function_prototype).expect("couldn't write to header file");
            h_file.as_ref().unwrap().write(b";\n"            ).expect("couldn't write to header file");
        }

    }

    println!(".h file updated.");
}
