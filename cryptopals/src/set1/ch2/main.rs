//  Cryptopals Set 1 Challenge 2
//  Josh Thompson, 2018
//
extern crate cryptopals;

use std::env;
use cryptopals::conversion::{hex_string_to_bytes, bytes_to_hex_string};
use cryptopals::xor::fixed_xor;

fn main() {

    println!("Cryptopals Set 1 Challenge 2 - Fixed xor");

    //get command line args
    let args: Vec<String> = env::args().collect();

    //Have two vectors of bytes
    let input1 = hex_string_to_bytes(&args[1]);
    let input2 = hex_string_to_bytes(&args[2]);

    let output = bytes_to_hex_string(&fixed_xor(&input1, &input2));

    println!("{:?}", &output);
}
