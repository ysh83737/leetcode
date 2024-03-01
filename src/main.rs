fn main() {
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 0), 0);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 4), 2);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 6), 3);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 7), 4);
    assert_eq!(Solution::search_insert(vec![1,3,5,6], 2), 1);
    assert_eq!(Solution::search_insert(vec![], 2), 0);
}

struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len() as i32;
        let mut left= 0;
        let mut right = n - 1;

        while left <= right {
            let mid = left + ((right - left) >> 1);
            let mid_num = nums[mid as usize];

            if target == mid_num {
                return mid;
            } else if target < mid_num {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        right + 1
    }
}
