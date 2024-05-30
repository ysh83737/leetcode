fn main() {
    assert_eq!(Solution::find_lhs(vec![1,3,2,2,5,2,3,7]), 5);
    assert_eq!(Solution::find_lhs(vec![1,2,3,4]), 2);
    assert_eq!(Solution::find_lhs(vec![1,1,1,1]), 0);
    assert_eq!(Solution::find_lhs(vec![1,3,5,7,9]), 0);
    assert_eq!(Solution::find_lhs(vec![1,3,3,3,3,4,4]), 6);
    assert_eq!(Solution::find_lhs(vec![1,2,2,1]), 4);
    assert_eq!(Solution::find_lhs(vec![1]), 0);
}

struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        nums.iter().for_each(|num| {
            map.entry(num).and_modify(|e| *e += 1).or_insert(1);
        });

        let mut result = 0;
        nums.iter().for_each(|num| {
            if let Some(&count) = map.get(&(num - 1)) {
                result = result.max(count + map.get(num).unwrap());
            }
        });
        
        result
    }
}
