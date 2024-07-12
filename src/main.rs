fn main() {
    assert_eq!(Solution::count_bits(2), vec![0,1,1]);
    assert_eq!(Solution::count_bits(5), vec![0,1,1,2,1,2]);
}

struct Solution;
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![0; n as usize + 1];
        for i in 1..=(n as usize) {
            result[i] = result[i & (i - 1)] + 1;
        }
        result
    }
}