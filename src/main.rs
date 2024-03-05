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
        while index >= 0 && i >=0 && j >= 0 {
            let num1 = nums1[i as usize];
            let num2 = nums2[j as usize];
            if num1 > num2 {
                nums1[index as usize] = num1;
                index -= 1;
                i -= 1;
            } else if num1 < num2 {
                nums1[index as usize] = num2;
                index -= 1;
                j -= 1;
            } else {
                nums1[index as usize] = num1;
                index -= 1;
                i -= 1;
                nums1[index as usize] = num2;
                index -= 1;
                j -= 1;
            }
        }
        while j >= 0 {
            let num2 = nums2[j as usize];
            nums1[index as usize] = num2;
            index -= 1;
            j -= 1;
        }
    }
}
