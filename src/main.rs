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

        let mut sum = 0;
        for i in 0..k {
            sum += nums[i as usize];
        }

        let mut max_sum = sum;
        for i in (k as usize)..n {
            sum += nums[i] - nums[i - k as usize];
            max_sum = max_sum.max(sum);
        }
        max_sum
    }
}