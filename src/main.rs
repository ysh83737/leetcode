fn main() {
    assert_eq!(Solution::reverse_bits(0b00000010100101000001111010011100), 964176192);
}

struct Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut result = 0;
        for i in 0..32 {
            result |= (x & 1) << (32 - i - 1);
            x = x >> 1;
        }
        result
    }
}