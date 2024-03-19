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
        for mut num in nums {
            let o_num = Some(num);
            if o_num == max1 || o_num == max2 || o_num == max3 {
                continue;
            }
            if max1 == None {
                max1 = Some(num);
                continue;
            } else if let Some(n) = max1 {
                if num > n {
                    max1 = Some(num);
                    num = n;
                }
            }
            if max2 == None {
                max2 = Some(num);
                continue;
            } else if let Some(n) = max2 {
                if num > n {
                    max2 = Some(num);
                    num = n;
                }
            }
            if max3 == None {
                max3 = Some(num);
                continue;
            } else if let Some(n) = max3 {
                if num > n {
                    max3 = Some(num);
                }
            }
        }
        if max3 == None {
            return max1.unwrap();
        }
        max3.unwrap()
    }
}
