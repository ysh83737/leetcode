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
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut sorted_nums = nums;
        sorted_nums.sort();

        let mut positive_nums = vec![];
        let mut negative_nums = vec![];

        for num in sorted_nums {
            if num > 0 {
                positive_nums.push(num);
            } else {
                // 0
                negative_nums.push(num);
            }
        }

        match positive_nums.len() {
            0 => {
                negative_nums.into_iter().rev().take(3).reduce(|acc, e| acc * e).unwrap()
            },
            1 => {
                positive_nums[0] * negative_nums.into_iter().take(2).reduce(|acc, e| acc * e).unwrap()
            },
            2 => {
                let last = positive_nums[positive_nums.len() - 1];
                last * negative_nums.into_iter().take(2).reduce(|acc, e| acc * e).unwrap()
            },
            _ => {
                let last = positive_nums[positive_nums.len() - 1];
                let mut max = positive_nums.into_iter().rev().take(3).reduce(|acc, e| acc * e).unwrap();
                max = max.max(last * negative_nums.into_iter().take(2).reduce(|acc, e| acc * e).unwrap_or(0));
                max
            }
        }
    }
}
