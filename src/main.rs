fn main() {
    assert_eq!(Solution::majority_element(vec![2,2,1,1,1,2,2]), 2);
}

struct Solution {}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut count = 0;
        for num in nums {
            if count == 0 {
                result = num;
            }
            if num == result {
                count += 1;
            } else {
                count -= 1;
            }
        }
        result
    }
}
