fn main() {
    assert_eq!(Solution::missing_number(vec![3,0,1]), 2);
}

struct Solution {}
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut result = 0;
        for i in 1..=n {
            result ^= i ^ nums[(i - 1) as usize];
        }
        result
    }
}
