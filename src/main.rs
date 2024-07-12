fn main() {
    assert_eq!(Solution::is_power_of_two(1), true);
    assert_eq!(Solution::is_power_of_two(16), true);
    assert_eq!(Solution::is_power_of_two(3), false);
    assert_eq!(Solution::is_power_of_two(536870912), true);
}

struct Solution;
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let log_2 = (n as f64).log(2.0);
        // f64精度误差
        log_2 - log_2.floor() < 1e-14
    }
}