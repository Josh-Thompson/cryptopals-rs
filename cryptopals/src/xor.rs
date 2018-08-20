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
