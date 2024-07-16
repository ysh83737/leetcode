fn main() {
    assert_eq!(Solution::find_complement(5), 2);
    assert_eq!(Solution::find_complement(1), 0);
    assert_eq!(Solution::find_complement(2147483647), 0);
}

struct Solution;
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut prefix = -1;
        let mut n = num;
        while n > 0 {
            prefix <<= 1;
            n >>= 1;
        }
        prefix |= num;
        !prefix
    }
}