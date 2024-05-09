use std::collections::{HashMap, VecDeque};

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
        let mut stack = VecDeque::new();
        let n = nums.len();
        for (i, &num) in nums.repeat(2).iter().enumerate().rev() {
            let index = i % n;
            while let Some(&top) = stack.front() {
                if top > num {
                    ans_map.insert(index, top);
                    break;
                } else {
                    stack.pop_front();
                }
            }
            stack.push_front(num);
        }

        nums.iter().enumerate().map(|(i, _)| {
            if let Some(&ans) = ans_map.get(&i) {
                return ans;
            }
            -1
        }).collect()
    }
}
