fn main() {
    assert_eq!(Solution::to_hex(26), "1a");
    assert_eq!(Solution::to_hex(0), "0");
    assert_eq!(Solution::to_hex(-1), "ffffffff");
}

struct Solution;
const DIGIT: u32 = (1 << 4) - 1;
const HEX_SYMBOLS: [char; 6] = ['a', 'b', 'c', 'd', 'e', 'f'];
impl Solution {
    pub fn to_hex(num: i32) -> String {
        // 转为无符号u32
        let mut u_num = num as u32;
        let mut result = String::new();
        while u_num != 0 {
            let value = u_num & DIGIT;
            let digit = if value < 10 {
                char::from_digit(value, 10).unwrap()
            } else {
                HEX_SYMBOLS[value as usize % 10]
            };
            result.push(digit);
            u_num >>= 4;
        }
        if result.len() == 0 {
            result.push('0');
        }
        result.chars().rev().collect()
    }
}