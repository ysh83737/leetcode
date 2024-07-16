fn main() {
    assert_eq!(Solution::hamming_distance(2, 4), 2);
    assert_eq!(Solution::hamming_distance(1, 3), 1);
    assert_eq!(Solution::hamming_distance(1577962638, 1727613287), 16);
}

struct Solution;
const M1: i32 = 0b01010101010101010101010101010101;
const M2: i32 = 0b00110011001100110011001100110011;
const M4: i32 = 0b00001111000011110000111100001111;
const M8: i32 = 0b00000000111111110000000011111111;
const M16: i32 = 0b00000000000000001111111111111111;
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut ret = x ^ y;
        ret = (ret & M1) + (ret >> 1 & M1);
        ret = (ret & M2) + (ret >> 2 & M2);
        ret = (ret & M4) + (ret >> 4 & M4);
        ret = (ret & M8) + (ret >> 8 & M8);
        ret = (ret & M16) + (ret >> 16 & M16);
        ret
    }
}