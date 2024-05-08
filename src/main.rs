use std::collections::HashMap;

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
        let mut stack = vec![];
        let mut i = nums2.len();
        while i > 0 {
            let num = nums2[i - 1];
            let mut j = stack.len();
            while j > 0 {
                let s_num = stack[j - 1];
                if s_num > num {
                    // 入栈
                    stack.push(num);
                    // 记录答案
                    ans_map.insert(num, s_num);
                    break;
                } else {
                    stack.pop();
                }
                j -= 1;
            }
            if j == 0 {
                // 空栈 入栈
                stack.push(num);
            }
            i -= 1;
        }

        let mut result = vec![];
        for num in nums1 {
            let ans = ans_map.get(&num).unwrap_or(&-1);
            result.push(*ans);
        }
        result
    }
}
