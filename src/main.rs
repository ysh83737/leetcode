fn main() {
    assert_eq!(Solution::find_error_nums(vec![1,2,2,4]), vec![2,3]);
    assert_eq!(Solution::find_error_nums(vec![1,1]), vec![1,2]);

}

struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;

        let mut map = std::collections::HashMap::new();
        for num in nums {
            map.entry(num).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut result = vec![0, 0];
        for i in 1..=n {
            match map.get(&i) {
                Some(&2) => result[0] = i,
                None => result[1] = i,
                _ => ()
            }
        }
        
        result
    }
}