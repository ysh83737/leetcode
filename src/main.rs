fn main() {
    assert_eq!(Solution::count_bits(2), vec![0,1,1]);
    assert_eq!(Solution::count_bits(5), vec![0,1,1,2,1,2]);
}

struct Solution;
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![0; n as usize + 1];
        let mut high_bit = 0;
        for i in 1..=n {
            if i & (i - 1) == 0 {
                high_bit = i;
            }
            result[i as usize] = result[(i - high_bit) as usize] + 1;
        }
        result
    }
}