//  Cryptopals Set 1 Challenge 4
//  Josh Thompson, 2018
//

use std::str;

extern crate cryptopals;

use cryptopals::xor::{byte_xor, find_byte_xor_key};
use cryptopals::conversion::hex_string_to_bytes;
use cryptopals::filereading::read_file;

fn main() {

    println!("Cryptopals Set 1 Challenge 4 - Detect single character XOR");

    //file setup
    let file_contents = read_file("data/4.txt");

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
