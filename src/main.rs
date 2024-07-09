fn main() {
    assert_eq!(Solution::find_error_nums(vec![1,2,2,4]), vec![2,3]);

}

struct Solution;

impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.sort();

        let mut result = vec![0, 0];
        let mut prev = 0;
        for &num in nums.iter() {
            if num == prev {
                result[0] = num;
            } else if num != prev + 1 {
                result[1] = prev + 1;
            }
            prev = num;
        }
        if nums[n - 1] != n as i32 {
            result[1] = n as i32;
        }
        
        result
    }
}