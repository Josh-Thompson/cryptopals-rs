// TODO switch to a more bit-based implementation to avoid all the wasted space from using u8s
//      and all the cutting off at the ends

///converts one hex character to a byte, leaving 4 front bits unused
//TODO should output 4 bits
fn hex_char_to_byte(c: char) -> u8 {
    match c {
        '0'...'9' => c as u8 - '0' as u8,
        'a'...'f' => 10 as u8 + c as u8 - 'a' as u8,
        _ => panic!("char_to_hex received non-hex input {}", c),
    }
}

//converts half a u8 (4 front bits must be unused) into a hex char
//TODO should input 4 bits
fn byte_to_hex_char(b: u8) -> char {
    match b {
        0...9 => (b + '0' as u8) as char,
        10...15 => (b - 10 + 'a' as u8) as char,
        _ => panic!("byte_to_hex_chars received input > 15: {}", b),
    }
}

///converts two characters from the ranges '0 - 9' and 'a - f' into a u8
//TODO should get deleted
fn hex_chars_to_byte(c1: char, c2: char) -> u8 {

    //gets 4 bits from the first hex char
    let a = hex_char_to_byte(c1);

    //gets 4 bits from the second hex char
    let b = hex_char_to_byte(c2);

    //puts the four bits of the two hex chars each together into one byte
    (a << 4) + b
}

//Converts a hex string into bytes
pub fn hex_string_to_bytes(string: &str) -> Vec<u8> {

    //Storage for our resulting bytes / u8s
    let mut bytes_vec: Vec<u8> = Vec::new();

    //Get an iterator over the characters of the string
    let mut hex = string.chars();

    //Consume our iterator
    loop {
        //Two characters are needed to make a byte from hex, so if we find 0 or 1 we stop.
        //This means trailing characters from odd-length hex strings will get removed.
        //Convert the hex characters to a byte, using their combined 8 bits
        let b1 = match hex.next() {
            None => break,
            Some(b) => b,
        };

        let b2 = match hex.next() {
            None => break,
            Some(b) => b,
        };

        bytes_vec.push(hex_chars_to_byte(b1, b2));
    }

    bytes_vec
}

//Converts a slice of bytes into a hex string
pub fn bytes_to_hex_string(bytes: &[u8]) -> String {

    //Every byte is two hex digits
    let mut string = String::with_capacity(bytes.len() * 2);

    for b in bytes {
        string.push(byte_to_hex_char(b >> 4));
        string.push(byte_to_hex_char(b & 15));
    }

    string
}

///The characters of base 64
const BASE_64: [char; 64] = [
    'A','B','C','D','E','F','G','H',
    'I','J','K','L','M','N','O','P',
    'Q','R','S','T','U','V','W','X',
    'Y','Z','a','b','c','d','e','f',
    'g','h','i','j','k','l','m','n',
    'o','p','q','r','s','t','u','v',
    'w','x','y','z','0','1','2','3',
    '4','5','6','7','8','9','+','/'
];

///Converts a slice of bytes to a B64 String
//TODO just rewrite this
pub fn bytes_to_base64_string(bytes: &[u8]) -> String {

    //let mut string = String::with_capacity(bytes.len() * 4 / 3);
    let mut string = String::new();

    let mut bytes_iter = bytes.iter();

    loop {
        let b1 = match bytes_iter.next() {
            Some(b) => b,
            None => break,
        };

        let b2 = match bytes_iter.next() {
            Some(b) => b,
            None => break,
        };

        let b3 = match bytes_iter.next() {
            Some(b) => b,
            None => break,
        };

        //between b1, b2, and b3, we now have 24 bits of data sorted into 8 bit chunks
        //we need to get that as 24 bits sorted into 4 6 bit chunks.
        //this ugliness is why I need to switch to using pure bits.
        let b64_1 = b1 >> 2;
        let b64_2 = ((b1 << 4) & 63u8) + ((b2 >> 4) & 63u8);
        let b64_3 = ((b2 << 2) & 63u8) + ((b3 >> 6) & 63u8);
        let b64_4 = b3 & 63u8;

        string.push(BASE_64[b64_1 as usize]);
        string.push(BASE_64[b64_2 as usize]);
        string.push(BASE_64[b64_3 as usize]);
        string.push(BASE_64[b64_4 as usize]);
    }

    string
}

pub fn base64_string_to_bytes(string: &str) -> Vec<u8> {

    let mut bytes_vec: Vec<u8> = Vec::new();

    let no_newlines = string.replace("\r\n", "").replace("\n", "");
    let mut b64_iter = no_newlines.chars();

    loop {
        let b1 = match b64_iter.next() {
            Some(b) => b,
            None => break,
        };

        let b2 = match b64_iter.next() {
            Some(b) => b,
            None => break,
        };

        let b3 = match b64_iter.next() {
            Some(b) => b,
            None => break,
        };

        let b4 = match b64_iter.next() {
            Some(b) => b,
            None => break,
        };

        //between each 4 characters we have 3 bytes of data.
        let key_1 = match BASE_64.iter().position(|&x| x == b1) {
            Some(b) => b,
            None => break,
        };

        let key_2 = match BASE_64.iter().position(|&x| x == b2) {
            Some(b) => b,
            None => break,
        };

        let key_3 = match BASE_64.iter().position(|&x| x == b3) {
            Some(b) => b,
            None => break,
        };

        let key_4 = match BASE_64.iter().position(|&x| x == b4) {
            Some(b) => b,
            None => break,
        };

        let byte_1 = (key_1 << 2) + (key_2 >> 4);
        let byte_2 = (key_2 << 4) + (key_3 >> 2);
        let byte_3 = (key_3 << 6) + (key_4 >> 0);

        //println!("{}\t{}\t{}\t{}", b1, b2, b3, b4);
        //println!("{}\t{}\t{}", byte_1, byte_2, byte_3);

        bytes_vec.push(byte_1 as u8);
        bytes_vec.push(byte_2 as u8);
        bytes_vec.push(byte_3 as u8);
    }

    bytes_vec
}
//Given input from Cryptopals challenge
#[test]
fn test_conversion() {

    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", bytes_to_base64_string(&hex_string_to_bytes(input)));
}
