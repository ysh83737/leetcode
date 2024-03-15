fn main() {
    assert_eq!(Solution::missing_number(vec![3,0,1]), 2);
}

struct Solution {}
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut sum = 0;
        for num in nums {
            sum += num;
        }
        n * (n + 1) / 2 - sum
    }
}
