use std::collections::HashMap;

fn main() {
    assert_eq!(Solution::next_greater_element(vec![4,1,2], vec![1,3,4,2]), vec![-1,3,-1]);
    assert_eq!(Solution::next_greater_element(vec![2,4], vec![1,2,3,4]), vec![3, -1]);
}

struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        let len2 = nums2.len();

        let mut nums2_map = HashMap::new();
        for i in 0..len2 {
            let num = nums2[i];
            nums2_map.insert(num, i);
        }

        let greater = |mut i: usize| {
            let num = nums2[i];
            i += 1;
            while i < len2 {
                let n = nums2[i];
                if n > num {
                    return n;
                }
                i += 1;
            }
            -1
        };
        for num in nums1 {
            let i = nums2_map.get(&num).cloned().unwrap();
            ans.push(greater(i));
        }

        ans
    }
}
