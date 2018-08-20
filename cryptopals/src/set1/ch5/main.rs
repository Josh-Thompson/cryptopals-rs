//  Cryptopals Set 1 Challenge 5
//  Josh Thompson, 2018
//

extern crate cryptopals;

use cryptopals::xor::repeat_xor;
use cryptopals::conversion::bytes_to_hex_string;

fn main() {

    println!("Cryptopals Set 1 Challenge 5 - Repeating key XOR");

    println!("{:?}", bytes_to_hex_string(&repeat_xor(b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", b"ICE")));

}
