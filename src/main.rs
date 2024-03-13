fn main() {
    assert_eq!(Solution::summary_ranges(vec![] as Vec<i32>), vec![] as Vec<String>);
    assert_eq!(Solution::summary_ranges(vec![0,1,2,4,5,7]), vec!["0->2","4->5","7"]);
    assert_eq!(Solution::summary_ranges(vec![0,2,3,4,6,8,9]), vec!["0","2->4","6","8->9"]);
}

struct Solution {}
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = vec![];
        if nums.len() == 0 {
            return result;
        }
        let mut start = nums[0];
        let mut end = nums[0];
        for i in 1..nums.len() {
            let num = nums[i];
            if num > end + 1 {
                Solution::push_range(&mut result, start, end);
                start = num;
            }
            end= num;
        }
        Solution::push_range(&mut result, start, end);
        result
    }
    fn push_range(ranges: &mut Vec<String>, start: i32, end: i32) -> () {
        if start == end {
            ranges.push(format!("{}", start));
        } else {
            ranges.push(format!("{}->{}", start, end));
        }
    }
}
