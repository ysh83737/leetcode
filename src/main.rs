fn main() {
    let nums1 = vec![1,2,2,1];
    let nums2 = vec![2,2];
    assert_eq!(Solution::intersect(nums1,nums2), vec![2,2]);
}

struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1.clone();
        let mut nums2 = nums2.clone();
        nums1.sort();
        nums2.sort();
        // 基于排序的双指针实现

        let mut result = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            let (n1, n2) = (nums1[i], nums2[j]);
            if n1 == n2 {
                result.push(n1);
                i += 1;
                j += 1;
            } else if n1 > n2 {
                j += 1;
            } else {
                i += 1;
            }
        }
        result
    }
}
