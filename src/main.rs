fn main() {
    let mut input = vec![1,1,2];
    println!(
        "{}",
        Solution::remove_duplicates(&mut input),
    );
}

struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut slow = 0;
        for fast in 0..nums.len() {
            let item_fast = nums[fast];
            let item_slow = nums[slow];
            if item_fast != item_slow {
                slow += 1;
                nums[slow] = item_fast;
            }
        }
        (slow + 1) as i32
    }
}
