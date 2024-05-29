use std::collections::HashSet;

fn main() {
    assert_eq!(Solution::distribute_candies(vec![1,1,2,2,3,3]), 3);
    assert_eq!(Solution::distribute_candies(vec![1,1,2,3]), 2);
    assert_eq!(Solution::distribute_candies(vec![6,6,6,6]), 1);
}

struct Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let n = candy_type.len();
        let half_n = n / 2;

        let mut eaten = HashSet::new();
        for candy in candy_type {
            eaten.insert(candy);
            if eaten.len() == half_n {
                break;
            }
        }

        eaten.len() as i32
    }
}
