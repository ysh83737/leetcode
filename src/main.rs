fn main() {
    assert_eq!(Solution::is_power_of_two(0), false);
    assert_eq!(Solution::is_power_of_two(1), true);
    assert_eq!(Solution::is_power_of_two(16), true);
    assert_eq!(Solution::is_power_of_two(3), false);
    assert_eq!(Solution::is_power_of_two(536870912), true);
}

struct Solution;

const BIG: i32 = 1 << 30;
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && BIG % n == 0
    }
}