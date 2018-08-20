use std::str;

//TODO ensure same lengths
pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {

    let mut result: Vec<u8> = Vec::new();

    for (i, j) in a.iter().zip(b.iter()) {

        result.push(i ^ j);
    }

    result
}

pub fn byte_xor(a: &[u8], b: u8) -> Vec<u8> {

    let mut result: Vec<u8> = Vec::new();

    for i in a.iter() {

        result.push(i ^ b);
    }

    result
}

//given a set of bytes XORed with a single unknown character, return the most likely character
//discerns based on highest percentage of alphabetic characters
pub fn find_byte_xor_key(input: &[u8]) -> u8 {

    let mut highscore: f64 = 0f64;
    let mut highscore_ind: u8 = 0u8;

    for x in 0..=255 {
        let xored = byte_xor(&input, x);
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

    highscore_ind
}

//Given input from Cryptopals challenge
#[test]
fn test_fixed_xor() {
    assert_eq!(
        fixed_xor(
            &hex_string_to_bytes("1c0111001f010100061a024b53535009181c")[..],
            &hex_string_to_bytes("686974207468652062756c6c277320657965")[..]
        ),
        &hex_string_to_bytes("746865206b696420646f6e277420706c6179")[..]
    )
}
