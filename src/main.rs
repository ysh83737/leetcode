fn main() {
    assert_eq!(Solution::maximum_product(vec![-1,-2,-3]), -6);
    assert_eq!(Solution::maximum_product(vec![-1,-2,-3,0]), 0);
    assert_eq!(Solution::maximum_product(vec![-1,-2,-3,1]), 6);
    assert_eq!(Solution::maximum_product(vec![-1,-2,-3,0,1]), 6);
    assert_eq!(Solution::maximum_product(vec![-1,-2,-3,0,1,2]), 12);
    assert_eq!(Solution::maximum_product(vec![0,1,2]), 0);
    assert_eq!(Solution::maximum_product(vec![-3,0,1,2]), 0);
    assert_eq!(Solution::maximum_product(vec![1,2,3]), 6);
    assert_eq!(Solution::maximum_product(vec![1,2,3,4]), 24);
    assert_eq!(Solution::maximum_product(vec![0,1,2,3]), 6);
    assert_eq!(Solution::maximum_product(vec![-3,0,1,2,3]), 6);
    assert_eq!(Solution::maximum_product(vec![-3,-2,0,1,2,3]), 18);

}

struct Solution;

impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort();

        
        let positive = nums[n - 1] * nums[n - 2] * nums[n - 3];
        positive.max(nums[0] * nums[1] * nums[n- 1])
    }
}
