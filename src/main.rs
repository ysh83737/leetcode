use std::{cmp::min, collections::HashSet};

fn main() {
    assert_eq!(Solution::distribute_candies(vec![1,1,2,2,3,3]), 3);
    assert_eq!(Solution::distribute_candies(vec![1,1,2,3]), 2);
    assert_eq!(Solution::distribute_candies(vec![6,6,6,6]), 1);
}

struct Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let n = candy_type.len();
        let candy_set: HashSet<i32> = HashSet::from_iter(candy_type);
        min(candy_set.len(), n / 2) as i32
    }
}
