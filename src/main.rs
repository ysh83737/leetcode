fn main() {
    assert_eq!(Solution::find_error_nums(vec![1,2,2,4]), vec![2,3]);
    assert_eq!(Solution::find_error_nums(vec![1,1]), vec![1,2]);
    assert_eq!(Solution::find_error_nums(vec![2,2]), vec![2,1]);
    assert_eq!(Solution::find_error_nums(vec![3,2,2]), vec![2,1]);

}

struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        let n_sum = n * (n + 1) / 2;
        let sum: i32 = nums.iter().sum();
        let set_sum: i32 = nums.into_iter().collect::<std::collections::HashSet<_>>().iter().sum();
        vec![sum - set_sum, n_sum - set_sum]
    }
}
