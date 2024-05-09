use std::collections::HashMap;

fn main() {
    assert_eq!(Solution::next_greater_elements(vec![1,2,1]), vec![2,-1,2]);
    assert_eq!(Solution::next_greater_elements(vec![1,2,3,4,3]), vec![2,3,4,-1,4]);
    assert_eq!(Solution::next_greater_elements(vec![100,1,11,1,120,111,123,1,-1,-100]), vec![120,11,120,120,123,123,-1,100,100,100]);
}

struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        // nums中所有索引对应的答案Map
        let mut ans_map = HashMap::new();

        // 单调栈
        let mut stack = vec![];
        let n = nums.len();
        let mut i = n * 2;
        while i > 0 {
            let index = (i - 1) % n;
            let num = nums[index];

            let mut j = stack.len();
            while j > 0 {
                let top = stack[j - 1];
                if top > num {
                    ans_map.insert(index, top);
                    break;
                } else {
                    stack.pop();
                }
                j -= 1;
            }
            stack.push(num);
            i -= 1;
        }

        let mut result = vec![];
        for i in 0..n {
            if let Some(&ans) = ans_map.get(&i) {
                result.push(ans);
            } else {
                result.push(-1);
            }
        }
        result
    }
}
