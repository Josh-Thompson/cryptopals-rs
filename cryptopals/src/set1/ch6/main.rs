//  Cryptopals Set 1 Challenge 6
//  Josh Thompson, 2018
//
use std::str;
use std::cmp::Ordering::Less;

extern crate cryptopals;

use cryptopals::compute::hamming_distance;
use cryptopals::conversion::base64_string_to_bytes;
use cryptopals::filereading::read_file;
use cryptopals::xor::{find_byte_xor_key, repeat_xor};

fn main() {

    println!("Cryptopals Set 1 Challenge 6 - Break repeating key XOR");

    let min_key_size = 2;
    let max_key_size = 40;
    let most_key_lengths_checked = 5;

    let encrypted_contents = base64_string_to_bytes(&read_file("data/6.txt"));
    let file_contents = str::from_utf8(&encrypted_contents).expect("Bad utf8!!");
    let mut winners: Vec<(f32, usize)> = Vec::new();

    //loop through potential key sizes
    for keysize in min_key_size..max_key_size {

        let chunk_1 = &file_contents[(0 * keysize)..(1 * keysize)];
        let chunk_2 = &file_contents[(1 * keysize)..(2 * keysize)];
        let chunk_3 = &file_contents[(2 * keysize)..(3 * keysize)];
        let chunk_4 = &file_contents[(3 * keysize)..(4 * keysize)];

        let average_hamming_distance: f32 = (hamming_distance(chunk_1, chunk_2) as f32
            + hamming_distance(chunk_1, chunk_3) as f32
            + hamming_distance(chunk_1, chunk_4) as f32
            + hamming_distance(chunk_2, chunk_3) as f32
            + hamming_distance(chunk_2, chunk_4) as f32
            + hamming_distance(chunk_3, chunk_4) as f32) / 6f32;

        let normalized_hamming_distance = average_hamming_distance as f32 / keysize as f32;

        winners.push((normalized_hamming_distance, keysize));
    }

    winners.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Less));

    let mut keys: Vec<String> = Vec::new();

    for winner in winners.iter().take(most_key_lengths_checked) {

        println!("Checking key length {} which has edit distance {}:", winner.1, winner.0);
        let mut key: Vec<u8> = Vec::new();

        for offset in 0..winner.1 {

            let chunk: String = file_contents.chars().skip(offset).step_by(winner.1).collect();
            let key_char = find_byte_xor_key(chunk.as_bytes()).0;
            key.push(key_char);
        }

        println!("{:?}", str::from_utf8(&key).expect("Bad key!"));
        keys.push(str::from_utf8(&key).expect("Bad key!").to_string());
    }

    println!("Output from most likely key:\n\n{:?}", str::from_utf8(&repeat_xor(file_contents.as_bytes(), keys[0].as_bytes())).expect("Bad result!").replace("", ""));

}
