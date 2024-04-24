use std::cmp::max;

fn main() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![1,1,0,1,1,1]), 3);
    assert_eq!(Solution::find_max_consecutive_ones(vec![1,1,0,0,0,0,0,0]), 2);
    assert_eq!(Solution::find_max_consecutive_ones(vec![1,1,0,0,1,0]), 2);
    assert_eq!(Solution::find_max_consecutive_ones(vec![1,0,0,0]), 1);
    assert_eq!(Solution::find_max_consecutive_ones(vec![0,0,1,0,0,0]), 1);
    assert_eq!(Solution::find_max_consecutive_ones(vec![0,0,0,0]), 0);
}

struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut temp = 0;
        for num in nums {
            if num == 1 {
                temp += 1;
            } else {
                temp = 0;
            }
            result = max(result, temp);
        }
        result
    }
}
