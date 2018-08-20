//  Cryptopals Set 1 Challenge 3
//  Josh Thompson, 2018
//
extern crate cryptopals;

use std::env;
use std::str;
use cryptopals::conversion::hex_string_to_bytes;
use cryptopals::xor::byte_xor;

fn main() {

    println!("Cryptopals Set 1 Challenge 3 - Single byte xor cipher");

    //get command line args
    let args: Vec<String> = env::args().collect();

    //Have two vectors of bytes
    let input1 = hex_string_to_bytes(&args[1]);

    let mut highscore: f64 = 0f64;
    let mut highscore_ind: u8 = 0u8;

    for x in 0..=255 {
        let xored = byte_xor(&input1, x);
        let decoded = str::from_utf8(&xored);
        if !decoded.is_err() {

            let decoded = decoded.unwrap();
            let mut char_count: u32 = 0;

            for y in decoded.chars() {

                if y.is_alphabetic() {

                    char_count += 1;
                }
            }

            let score: f64 = char_count as f64 / decoded.len() as f64;

            if score > highscore {

                highscore = score;
                highscore_ind = x;
            }
        }
    }

    println!("PHRASE: {}", str::from_utf8(&byte_xor(&input1[..], highscore_ind)).unwrap());
    println!("KEY: {}", highscore_ind as char);

}
