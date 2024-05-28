fn main() {
    assert_eq!(Solution::array_pair_sum(vec![1,4,3,2]), 4);
    assert_eq!(Solution::array_pair_sum(vec![6,2,6,5,1,2]), 9);
}

struct Solution;

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mut result = 0;
        let mut i = 0;
        while i < nums.len() {
            result += nums[i];
            i += 2;
        }
        result
    }
}
