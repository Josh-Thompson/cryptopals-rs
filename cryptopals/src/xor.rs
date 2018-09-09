use std::str;

//TODO ensure same lengths
///XORs two vectors of bytes of the same length against eachother
pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {

    let mut result: Vec<u8> = Vec::new();

    for (i, j) in a.iter().zip(b.iter()) {

        result.push(i ^ j);
    }

    result
}

///XORs a vector of bytes all against a single character and returns the result
pub fn byte_xor(a: &[u8], b: u8) -> Vec<u8> {

    let mut result: Vec<u8> = Vec::new();

    for i in a.iter() {

        result.push(i ^ b);
    }

    result
}

///given a set of bytes XORed with a single unknown character, return the most likely character
///also returns the score of said character, discerned based on highest percentage of alphabetic
///characters or spaces
pub fn find_byte_xor_key(input: &[u8]) -> (u8, f64) {

    let mut highscore: f64 = 0f64;
    let mut highscore_ind: u8 = 0u8;

    for x in 0..=255 {
        let xored = byte_xor(&input, x);
        let decoded = str::from_utf8(&xored);
        if !decoded.is_err() {

            let decoded = decoded.unwrap();
            let mut char_count: u32 = 0;

            for y in decoded.chars() {

                if y.is_alphabetic() || y == ' ' {

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

    (highscore_ind, highscore)
}

//Given a set of bytes, repeat xors those with a set of bytes as a repeating key
pub fn repeat_xor(input: &[u8], key: &[u8]) -> Vec<u8> {

    let mut result: Vec<u8> = Vec::new();

    let mut curr = 0;

    for x in input.iter() {

        result.push(x ^ key[curr]);

        curr += 1;

        if curr == key.len() {
            curr = 0;
        }
    }

    result
}

//Given input from Cryptopals challenge
#[test]
fn test_fixed_xor() {

    use conversion::hex_string_to_bytes;

    assert_eq!(
        fixed_xor(
            &hex_string_to_bytes("1c0111001f010100061a024b53535009181c")[..],
            &hex_string_to_bytes("686974207468652062756c6c277320657965")[..]
        ),
        &hex_string_to_bytes("746865206b696420646f6e277420706c6179")[..]
    )
}
