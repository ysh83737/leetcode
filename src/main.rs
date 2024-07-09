fn main() {
    assert_eq!(Solution::find_error_nums(vec![1,2,2,4]), vec![2,3]);
    assert_eq!(Solution::find_error_nums(vec![1,1]), vec![1,2]);
    assert_eq!(Solution::find_error_nums(vec![2,2]), vec![2,1]);
    assert_eq!(Solution::find_error_nums(vec![3,2,2]), vec![2,1]);

}

struct Solution;

impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0, 0];
        for i in 0..n {
            let num = nums[i];
            let value = num.abs();
            let index = (value - 1) as usize;
            if nums[index] < 0 {
                result[0] = value;
            } else {
                nums[index] = -nums[index];
            }
        }
        for i in 0..n {
            if nums[i] > 0 {
                result[1] = (i + 1) as i32;
            }
        }
        
        result
    }
}