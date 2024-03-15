fn main() {
    let mut nums = vec![0,1,0,3,12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1,3,12,0,0]);
}

struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut slow = 0;
        let mut fast = 0;
        while slow < n && fast < n {
            if nums[slow] != 0 {
                slow += 1;
            } else if nums[fast] != 0 {
                nums[slow] = nums[fast];
                nums[fast] = 0;
                slow += 1;
            }
            fast += 1;
        }
    }
}
