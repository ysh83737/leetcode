fn main() {
    assert_eq!(Solution::count_bits(2), vec![0,1,1]);
    assert_eq!(Solution::count_bits(5), vec![0,1,1,2,1,2]);
}

struct Solution;
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..=n {
            let mut num = i;
            let mut count_of_bit = 0;
            while num > 0 {
                count_of_bit += 1;
                num = num & (num - 1);
            }
            result.push(count_of_bit);
        }
        result
    }
}