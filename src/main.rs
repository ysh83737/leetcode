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

const MAX: i32 = 1000;
const MIN: i32 = -1000;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut min1 = MAX;
        let mut min2 = MAX;

        let mut max1 = MIN;
        let mut max2 = MIN;
        let mut max3 = MIN;
        
        for num in nums {
            if num < min1 {
                (min1, min2) = (num, min1);
            } else if num < min2 {
                min2 = num;
            }

            if num > max1 {
                (max1, max2, max3) = (num, max1, max2);
            } else if num > max2 {
                (max2, max3) = (num, max2);
            } else if num > max3 {
                max3 = num;
            }
        }

        
        let positive = max1 * max2 * max3;
        positive.max(max1 * min1 * min2)
    }
}
