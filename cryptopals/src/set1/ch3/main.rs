//  Cryptopals Set 1 Challenge 3
//  Josh Thompson, 2018
//
extern crate cryptopals;

use std::env;
use std::str;
use cryptopals::conversion::hex_string_to_bytes;
use cryptopals::xor::{find_byte_xor_key, byte_xor};

fn main() {

    println!("Cryptopals Set 1 Challenge 3 - Single byte xor cipher");

    //get command line args
    let args: Vec<String> = env::args().collect();

    //Have vector of bytes
    let input1 = hex_string_to_bytes(&args[1]);

    let key = find_byte_xor_key(&input1);

    println!("PHRASE: {}", str::from_utf8(&byte_xor(&input1[..], key)).unwrap());
    println!("KEY: {}", key as char);

}
