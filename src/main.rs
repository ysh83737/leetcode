fn main() {
    assert_eq!(Solution::single_number(vec![2,2,1]), 1);
}

struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            result ^= nums[i];
        }
        result
    }
}
