use std::collections::VecDeque;

fn main() {
    assert_eq!(Solution::next_greater_elements(vec![1,2,1]), vec![2,-1,2]);
    assert_eq!(Solution::next_greater_elements(vec![1,2,3,4,3]), vec![2,3,4,-1,4]);
    assert_eq!(Solution::next_greater_elements(vec![100,1,11,1,120,111,123,1,-1,-100]), vec![120,11,120,120,123,123,-1,100,100,100]);
}

struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![-1; n];
        let mut stack = VecDeque::new();
        for i in 0..(2 * n - 1) {
            let index = i % n;
            let num = nums[index];
            while stack.len() > 0 && nums[*stack.front().unwrap()] < num {
                let top = *stack.front().unwrap();
                result[top] = num;
                stack.pop_front();
            }
            stack.push_front(index);
        }

        result
    }
}
