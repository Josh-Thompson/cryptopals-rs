extern crate cryptopals;
extern crate openssl;

use cryptopals::filereading::read_file;
use cryptopals::conversion::base64_string_to_bytes;
use openssl::symm::{decrypt, Cipher};
use std::str;

fn main() {

    let cipher = Cipher::aes_128_ecb();
    let key = "YELLOW SUBMARINE";
    let file_contents = read_file("data/7.txt");
    let unb64 = base64_string_to_bytes(&file_contents);
    let text = decrypt(cipher, key.as_bytes(), None, &unb64).unwrap();
    println!("{}", str::from_utf8(&text).unwrap());
}
