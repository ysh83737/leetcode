fn main() {
    assert_eq!(Solution::find_complement(5), 2);
    assert_eq!(Solution::find_complement(1), 0);
    assert_eq!(Solution::find_complement(2147483647), 0);
}

struct Solution;
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut valid_bit = num;
        for i in [1, 2, 4, 8, 16] {
            valid_bit |= valid_bit >> i;
        }
        valid_bit & !num
    }
}