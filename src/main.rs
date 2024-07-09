fn main() {
    assert_eq!(Solution::find_error_nums(vec![1,2,2,4]), vec![2,3]);

}

struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        let mut sum = n * (n + 1) / 2;
        let mut set = std::collections::HashSet::new();
        let mut result = vec![];
        for num in nums {
            sum -= num;
            if set.contains(&num) {
                result.push(num);
            }
            set.insert(num);
        }
        result.push(result[0] + sum);
        result
    }
}