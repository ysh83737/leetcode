use std::collections::{HashMap, VecDeque};

fn main() {
    assert_eq!(Solution::next_greater_element(vec![4,1,2], vec![1,3,4,2]), vec![-1,3,-1]);
    assert_eq!(Solution::next_greater_element(vec![2,4], vec![1,2,3,4]), vec![3, -1]);
}

struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // nums2中所有值对应的答案Map
        let mut ans_map = HashMap::new();

        // 单调栈
        let mut stack = VecDeque::new();
        for &num in nums2.iter().rev() {
            while let Some(&top) = stack.front() {
                if top > num {
                    ans_map.insert(num, top);
                    break;
                } else {
                    stack.pop_front();
                }
            }
            stack.push_front(num);
        }

        nums1.iter().map(|x| *ans_map.get(x).unwrap_or(&-1)).collect()
    }
}
