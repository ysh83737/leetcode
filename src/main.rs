fn main() {
    assert_eq!(Solution::reverse_bits(0b00000010100101000001111010011100), 964176192);
}

struct Solution;

const M1: u32 = 0b10101010101010101010101010101010;
const M2: u32 = 0b11001100110011001100110011001100;
const M4: u32 = 0b11110000111100001111000011110000;
const M8: u32 = 0b11111111000000001111111100000000;
impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        x = M1 & x << 1 | (M1 & x) >> 1;
        x = M2 & x << 2 | (M2 & x) >> 2;
        x = M4 & x << 4 | (M4 & x) >> 4;
        x = M8 & x << 8 | (M8 & x) >> 8;
        x >> 16 | x << 16
    }
}