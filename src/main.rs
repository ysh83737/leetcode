fn main() {
    assert_eq!(Solution::find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]), vec![5,6]);
    assert_eq!(Solution::find_disappeared_numbers(vec![1,1]), vec![2]);
}

struct Solution;
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n {
            let num = nums[i];
            let index = (num - 1) as usize % n;
            nums[index] += n as i32;
        }
        println!("nums={:?}", nums);
        nums.into_iter().enumerate().filter_map(|(i, x)| {
            if x > n as i32 {
                return None
            }
            Some(i as i32 + 1)
        }).collect()
    }
}
