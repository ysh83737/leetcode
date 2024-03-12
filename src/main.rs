use std::collections::HashMap;

fn main() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1,2,3,1], 3), true);
}

struct Solution {}
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let num = nums[i];
            if let Some(j) = map.insert(num, i) {
                if i - j <= k as usize {
                    return true;
                }
            }
        }
        false
    }
}
