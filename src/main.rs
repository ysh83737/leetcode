fn main() {
    let nums1 = vec![1,2,2,1];
    let nums2 = vec![2,2];
    assert_eq!(Solution::intersection(nums1, nums2), vec![2]);
}

struct Solution;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for num in nums1 {
            set.insert(num);
        }
        let mut result = HashSet::new();
        for num in nums2 {
            if set.contains(&num) {
                result.insert(num);
            }
        }
        result.iter().map(|num| num * 1).collect()
    }
}
