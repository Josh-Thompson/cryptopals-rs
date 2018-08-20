//  Cryptopals Set 1 Challenge 1
//  Josh Thompson, 2018
//
extern crate cryptopals;

use std::env;
use cryptopals::conversion::{hex_string_to_bytes, bytes_to_base64_string};

fn main() {

    println!("Cryptopals Set 1 Challenge 1 - Convert Hex To B64");

    //get command line args
    let args: Vec<String> = env::args().collect();

    assert_eq!(args.len(), 2);

    //immutable reference to input
    let input = &args[1];

    println!("B64: {}", bytes_to_base64_string(&hex_string_to_bytes(input)));

}
