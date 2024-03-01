fn main() {
    let mut input = vec![3,2,2,3];
    println!(
        "{}",
        Solution::remove_element(&mut input, 3),
    );
}

struct Solution {}
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow = 0;
        for fast in 0..nums.len() {
            let curr = nums[fast];
            if curr != val {
                nums[slow] = curr;
                slow += 1;
            }
        }
        slow as i32
    }
}
