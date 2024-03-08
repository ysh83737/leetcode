fn main() {
    assert_eq!(Solution::majority_element(vec![2,2,1,1,1,2,2]), 2);
}

struct Solution {}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<i32, u32> = HashMap::new();
        let mut result = 0;
        let mut max_count = 0;
        for num in nums {
            let count = map.entry(num).or_insert(0);
            *count += 1;
            if *count > max_count {
                max_count = *count;
                result = num;
            }
        }
        result
    }
}
