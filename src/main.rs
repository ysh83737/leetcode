fn main() {
    assert_eq!(Solution::find_complement(5), 2);
    assert_eq!(Solution::find_complement(1), 0);
    assert_eq!(Solution::find_complement(2147483647), 0);
}

struct Solution;
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut ret = 0;
        let mut bit = 1;
        while bit > 0 && bit < num {
            if bit & num == 0 {
                ret ^= bit;
            }
            bit <<= 1;
        }
        ret
    }
}