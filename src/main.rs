fn main() {
    assert_eq!(Solution::is_power_of_two(1), true);
    assert_eq!(Solution::is_power_of_two(16), true);
    assert_eq!(Solution::is_power_of_two(3), false);
}

struct Solution;
impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        let mut bit_1 = false;
        while n > 0 {
            if n & 1 > 0 {
                if bit_1 {
                    return false;
                }
                bit_1 = true;
            }
            n >>= 1;
        }
        bit_1
    }
}