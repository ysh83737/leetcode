fn main() {
    assert_eq!(Solution::hamming_weight(11), 3);
}

struct Solution;

impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut result = 0;
        while n > 0 {
            result += n & 1;
            n = n >> 1;
        }
        result
    }
}