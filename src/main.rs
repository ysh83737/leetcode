fn main() {
    let nums1 = vec![1,2,2,1];
    let nums2 = vec![2,2];
    assert_eq!(Solution::intersection(nums1, nums2), vec![2]);
}

struct Solution;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut result = HashSet::new();
        let set: HashSet<i32> = nums1.into_iter().collect();
        for num in nums2 {
            if set.contains(&num) {
                result.insert(num);
            }
        }
        result.into_iter().collect()
    }
}
