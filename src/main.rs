fn main() {
    assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
    assert_eq!(Solution::third_max(vec![2, 1]), 2);
    assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    assert_eq!(Solution::third_max(vec![1,2,-2147483648]), -2147483648);
}

struct Solution;
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut max1 = None;
        let mut max2 = None;
        let mut max3 = None;
        for n in nums {
            let mut num = Some(n);
            if num == max1 || num == max2 || num == max3 {
                continue;
            }
            if num > max1 {
                (num, max1) = (max1, num);
            }
            if num > max2 {
                (num, max2) = (max2, num);
            }
            if num > max3 {
                max3 = num;
            }
        }
        match max3 {
            Some(n) => n,
            None => max1.unwrap()
        }
    }
}
