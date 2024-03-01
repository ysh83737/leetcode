fn main() {
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 2), 1);
}

struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for index in 0..nums.len() {
            let item = nums[index];
            if item >= target {
                return index as i32;
            }
        }
        nums.len() as i32
    }
}
