use std::str;

//calculates the number of differing bits between two strings
pub fn hamming_distance(a: &str, b: &str) -> u32 {

    //only works for equal length strings
    assert_eq!(a.len(), b.len());

    let mut dist: u32 = 0;

    //zips the two strings into an array of (x,y) tuples of bytes and iterates
    for i in a.as_bytes().iter().zip(b.as_bytes().iter()) {

        dist += hamming_weight((i.0 ^ i.1) as u32);
    }

    dist
}

//calculates the number of 'set bits' in a given integer. Don't ask me why this bullshit works.
fn hamming_weight(val: u32) -> u32 {
    let v1 = val - ((val >> 1) & 0x55555555);
    let v2 = (v1 & 0x33333333) + ((v1 >> 2) & 0x33333333);
    (((v2 + (v2 >> 4)) & 0xF0F0F0F).wrapping_mul(0x1010101)) >> 24
}

#[test]
fn test_hamming() {
    assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
}
