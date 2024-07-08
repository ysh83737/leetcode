fn main() {
    assert_eq!(Solution::find_max_average(vec![1,12,-5,-6,50,3], 4), 12.75);

}

struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let max_sum = Solution::find_max_sum(nums, k);
        max_sum as f64 / k as f64
    }
    pub fn find_max_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut i = 0;
        let mut j = 0;
        let mut sum = 0;
        let mut max_sum = 0;
        while j < n {
            sum += nums[j];
            if j < (k as usize) {
                max_sum = sum;
            } else {
                sum -= nums[i];
                max_sum = max_sum.max(sum);
                i += 1;
            }
            j += 1;
        }
        max_sum
    }
}