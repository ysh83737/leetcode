fn main() {
    let nums1 = vec![1,2,2,1];
    let nums2 = vec![2,2];
    assert_eq!(Solution::intersect(nums1,nums2), vec![2,2]);
}

struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() > nums2.len() {
            return  Solution::intersect(nums2, nums1);
        }
        use std::collections::HashMap;
        let mut map: HashMap<i32, u32> = HashMap::new();
        for num in nums1 {
            map.entry(num).and_modify(|count| { *count += 1; }).or_insert(1);
        }
        let mut result = vec![];
        for num in nums2 {
            map.entry(num).and_modify(|count| {
                if *count > 0 {
                    result.push(num);
                    *count -= 1;
                }
            });
        }
        result
    }
}
