fn main() {
    assert_eq!(Solution::summary_ranges(vec![] as Vec<i32>), vec![] as Vec<String>);
    assert_eq!(Solution::summary_ranges(vec![0,1,2,4,5,7]), vec!["0->2","4->5","7"]);
    assert_eq!(Solution::summary_ranges(vec![0,2,3,4,6,8,9]), vec!["0","2->4","6","8->9"]);
}

struct Solution {}
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let len = nums.len();
        let mut result = vec![];
        let mut i = 0;
        while i < len {
            let start = nums[i];
            while i < len - 1 && nums[i] + 1 == nums[i + 1] {
                i += 1;
            }
            let end = nums[i];
            if start == end {
                result.push(format!("{}", start));
            } else {
                result.push(format!("{}->{}", start, end));
            }
            i += 1;
        }
        result
    }
}
