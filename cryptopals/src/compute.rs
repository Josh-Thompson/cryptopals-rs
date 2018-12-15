use std::str;

//calculates the number of differing bits between two strings
pub fn hamming_distance(a: &str, b: &str) -> u32 {

    //only works for equal length strings
    assert_eq!(a.len(), b.len());

    let mut dist: u32 = 0;

    //zips the two strings into an array of (x,y) tuples of bytes and iterates
    for i in a.as_bytes().iter().zip(b.as_bytes().iter()) {

        dist += hamming_weight(i.0 ^ i.1) as u32;
    }

    dist
}

fn hamming_weight(val: u8) -> u8 {

    let mut res = 0;

    for bit_position in 0..8 {

        if (val >> bit_position) & 1u8 == 1u8 {

            res = res + 1;
        }
    }

    res
}

#[test]
fn test_hamming() {
    assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
}
