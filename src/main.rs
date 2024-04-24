fn main() {
    assert_eq!(Solution::find_poisoned_duration(vec![1,4], 2), 4);
    assert_eq!(Solution::find_poisoned_duration(vec![1,2], 2), 3);
}

struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut result = 0;
        // 中毒结束
        let mut end = 0;
        for t in time_series {
            result += duration;
            if t < end {
                result -= end - t;
            }
            end = t + duration
        }
        result
    }
}
