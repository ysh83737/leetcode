fn main() {
    assert_eq!(Solution::find_lhs(vec![1,3,2,2,5,2,3,7]), 5);
    assert_eq!(Solution::find_lhs(vec![1,2,3,4]), 2);
    assert_eq!(Solution::find_lhs(vec![1,1,1,1]), 0);
    assert_eq!(Solution::find_lhs(vec![1,3,5,7,9]), 0);
    assert_eq!(Solution::find_lhs(vec![1,3,3,3,3,4]), 5);
    assert_eq!(Solution::find_lhs(vec![1,2,2,1]), 4);
    assert_eq!(Solution::find_lhs(vec![1]), 0);
}

struct Solution;

impl Solution {
    pub fn find_lhs(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| a.cmp(b));

        let mut i = 0;
        let mut j = 0;
        let mut result = 0;

        while j < nums.len() {
            while i < j && nums[j] - nums[i] > 1 {
                i += 1;
            }
            if nums[j] - nums[i] == 1 {
                result = result.max(j - i + 1);
            }
            j += 1;
        }
        result as i32
    }
}
