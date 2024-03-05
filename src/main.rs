fn main() {
    let mut nums1 = vec![1,2,3,0,0,0];
    let mut nums2 = vec![2,5,6];
    Solution::merge(
        &mut nums1,
        3,
        &mut nums2,
        3,
    );
    println!("done");
    println!("nums1={:?}", nums1);
    let mut nums1 = vec![0];
    let mut nums2 = vec![1];
    Solution::merge(
        &mut nums1,
        0,
        &mut nums2,
        1,
    );
    println!("done");
    println!("nums1={:?}", nums1);
}

struct Solution {}
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut index = m + n - 1;
        let mut i = m - 1;
        let mut j = n - 1;
        let mut cur: i32;
        while i >=0 || j >= 0 {
            if j == -1 {
                break;
            } else if i == -1 {
                cur = nums2[j as usize];
                j -= 1;
            } else {
                let num1 = nums1[i as usize];
                let num2 = nums2[j as usize];
                if num1 > num2 {
                    cur = num1;
                    i -= 1;
                } else {
                    cur = num2;
                    j -= 1;
                }
            }
            nums1[index as usize] = cur;
            index -= 1;
        }
    }
}
