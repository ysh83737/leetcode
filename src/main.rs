fn main() {
    assert_eq!(Solution::hamming_distance(1, 4), 2);
    assert_eq!(Solution::hamming_distance(1, 3), 1);
}

struct Solution;
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut xor = x ^ y;
        let mut count = 0;
        while xor > 0 {
            count += 1;
            xor &= xor - 1;
        }
        count
    }
}