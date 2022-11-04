
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

/*      TODO

put on git
only add file if it contains // public tag
why doesn't basic test work with cargo test
add output checks to automated
*/


use std::fs::File;
use std::env;
use std::path::Path;
use std::io::{BufReader, BufRead, Write};


fn main() {

    // define constants

    let public_tag = "// public";
    let command_line_input_description = 
    "
    please use the below format
        header <path from current directory to c file>
        header linkedlist.c
        
    ";


    // process cli args

    let num_args = 1; // TODO derive, not hardcoded

    let mut args = env::args();
    let _cd = args.next().unwrap();

    assert!(args.len() == num_args, "\n\nWrong number of arguements provided\nprovided {} expected {}\n{}", args.len(), num_args, command_line_input_description);

    // open .c file

    let c_file_string = &args.next().unwrap();
    
    assert!(c_file_string.ends_with(".c"), "file does not have .c extension");

    let c_file_path = Path::new(&c_file_string);
    assert!(c_file_path.exists(), "c file does not exist in the current directory");

    let c_file = File::open(c_file_path)
        .expect("Cannot open file");

    // open new .h file with same name

    let h_file_string = c_file_string.replace(".c", "-functions.h");

    let h_file_path = Path::new(&h_file_string);

    let mut h_file = File::create(h_file_path)
        .expect("could not create header file");

    // 

    let c_file_name = c_file_path.file_stem().unwrap().to_str().unwrap().as_bytes();

    // #inlucde "list-structs.h"
    h_file.write(b"#include \"").unwrap();
    h_file.write(c_file_name).unwrap(); 
    h_file.write( b"-structs.h\"\n\n").unwrap();


    //

    let c_file_buffer_reader = BufReader::new(c_file);

    // iterate through the c file as a text file
    let mut was_public_tag = false;

    for result_line in c_file_buffer_reader.lines() {

        let line = result_line
            .expect("couldn't read line");

        // add header to the h file 
        if was_public_tag {

            let function_prototype = line[..line.find("{").unwrap()].as_bytes();

            h_file.write(function_prototype).expect("couldn't write to header file");
            h_file.write(b";\n"            ).expect("couldn't write to header file");

        }

        // when a public tag is found,
        was_public_tag = line.starts_with(public_tag);

    }

    println!(".h file updated.");

}
