fn main() {
    assert_eq!(Solution::is_power_of_four(16), true);
    assert_eq!(Solution::is_power_of_four(5), false);
    assert_eq!(Solution::is_power_of_four(1), true);
    assert_eq!(Solution::is_power_of_four(8), false);
    assert_eq!(Solution::is_power_of_four(0), false);
}

struct Solution;
const BITS: i32 = 0b01010101010101010101010101010101;
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n & BITS > 0 && n & (n - 1) == 0
    }
}