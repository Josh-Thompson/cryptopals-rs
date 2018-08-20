//  Cryptopals Set 1 Challenge 4
//  Josh Thompson, 2018
//

use std::fs::File;
use std::io::prelude::*;
use std::str;

extern crate cryptopals;

use cryptopals::xor::{byte_xor, find_byte_xor_key};
use cryptopals::conversion::hex_string_to_bytes;

fn main() {

    println!("Cryptopals Set 1 Challenge 4 - Detect single character XOR");

    //file setup
    let mut f = File::open("data/4.txt").expect("File not found");

    let mut file_contents = String::new();

    f.read_to_string(&mut file_contents).expect("Something went wrong reading the file");

    //variables for looping
    let mut highscore: f64 = 0f64;
    let mut winner = String::new();

    //go through all lines
    for line in file_contents.split('\n') {

        //find the most likely key for this line
        let (key, score) = find_byte_xor_key(&hex_string_to_bytes(&line));

        //get the resultant bytes from that xor
        let xored = byte_xor(&hex_string_to_bytes(&line), key);

        //get that as text
        let res = str::from_utf8(&xored);

        //if valid text, check how good the score is and set our current winner
        if !res.is_err() {

            if score > highscore {

                highscore = score;
                winner = res.unwrap().to_string();
            }
        }
    }

    //got 'em.
    println!("{}", winner);
}
