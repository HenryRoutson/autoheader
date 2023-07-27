use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn assert_same_file_contents(file_name1: String, file_name2: String) {
    // open file 1
    let path1 = Path::new(&file_name1);
    if !path1.exists() {
        panic!("\n\n{:#?}\n DOES NOT EXIST \n\n", path1.to_str())
    }

    let mut reader1 = BufReader::new(File::open(&path1).unwrap()).lines();

    // open file 2
    let path2 = Path::new(&file_name2);
    if !path2.exists() {
        panic!("\n\n{:#?}\n DOES NOT EXIST \n\n", path2.to_str())
    }

    assert!(path2.exists());
    let mut reader2 = BufReader::new(File::open(&path2).unwrap()).lines();

    // compare, line by line for easy debug
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

            assert!(line1 == line2, "\n\n{line1}\n != \n{line2}\n\n");
        }
    }
}

pub fn assert_correct_file_contents(file: String) {
    let correct_file = file.clone().replace(".", "-correct.");
    assert_same_file_contents(file, correct_file);
}
