use std::collections::HashMap;

fn main() {
    println!(
        "{:?}",
        Solution::two_sum(vec![2,7,11,15], 9),
    );
}

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let len = nums.len();
        for index in 0..len {
            let num = nums[index];
            let rest = target - num;

            if map.contains_key(&rest) {
                let v = map.get(&rest).unwrap();
                return vec![*v, index as i32];
            }
            map.insert(num, index as i32);
        }

        unreachable!()
    }
}
