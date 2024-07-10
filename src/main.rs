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

        let mut xor = 0;
        for i in 1..=n {
            xor ^= i ^ nums[i as usize - 1];
        }

        let lowbit = xor & -xor;
        let mut num1 = 0;
        let mut num2 = 0;
        let mut grouping = |num: i32| {
            if num & lowbit == 0 {
                num1 ^= num;
            } else {
                num2 ^= num;
            }
        };
        for i in 1..=n {
            grouping(i);
            grouping(nums[i as usize - 1]);
        }

        let mut result = vec![num1, num2];
        for num in nums {
            if num == num2 {
                result.reverse();
                break;
            }
        }
        result
    }
}
