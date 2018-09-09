use std::fs::File;
use std::str;
use std::io::prelude::*;

//gets the contents of a file given a name as a string
pub fn read_file(filename: &str) -> String {

    let mut f = File::open(filename).expect("File not found");

    let mut file_contents = String::new();

    f.read_to_string(&mut file_contents).expect("Something went wrong reading the file");

    file_contents
}
