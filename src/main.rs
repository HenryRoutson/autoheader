
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
use std::path::Path;
use std::io::{Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::fs;


fn main() {

    let public_tag = "// public";

    //
    
    print!("\n\n");

    let paths = fs::read_dir("./").unwrap();

    for path in paths {

        let c_file_path = path.unwrap().path();
        let c_file_string = c_file_path.to_str().unwrap();

        if c_file_string.ends_with(".c") {

            // open .c file contents++
            let c_file_path = Path::new(&c_file_string); 
            assert!(c_file_path.exists(), "c file does not exist");

            let mut c_file_content = String::new();
            File::open(c_file_path).expect("Cannot open file").read_to_string(&mut c_file_content).unwrap();
            if !c_file_content.contains(public_tag) {
                println!("  {} didn't contain any public tags, a functions file was not created", c_file_string);
                break;
            }

            // create h file
            let write_error = "Error writing to file";
            let h_file_string = c_file_string[..c_file_string.len() - 2].to_string() + &"-functions.h".to_string();  
            let h_file_path = Path::new(&h_file_string);
            let mut h_file = File::create(h_file_path).expect("could not create header file");
            println!("  {} functions file was created", c_file_string);

            // #include "****-structs.h" in functions.h for defined types
            h_file.write(b"#include \""                             ).expect(write_error);
            h_file.write(c_file_path.file_stem().unwrap().as_bytes()).expect(write_error);
            h_file.write( b"-structs.h\"\n\n"                       ).expect(write_error);

            for s in c_file_content.split(public_tag).skip(1) {
                let function_prototype = s[..s.find('{').unwrap()-1].trim().as_bytes();
                h_file.write(&function_prototype).expect(write_error);
                h_file.write(b";\n"             ).expect(write_error);
            } 
        }
    }

    println!("\nDone\n\n");
}
