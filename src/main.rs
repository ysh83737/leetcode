fn main() {
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 0), 0);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 4), 2);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 6), 3);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 7), 4);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 2), 1);
}

struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len();
        while right - left > 1 {
            let middle = left + (((right - left) / 2) as f64).round() as usize;
            let middle_num = nums[middle];

            if middle_num == target {
                return middle as i32;
            } else if middle_num > target {
                right = middle;
            } else {
                left = middle;
            }
        }

        if target > nums[left] {
            return right as i32;
        }
        left as i32
    }
}
