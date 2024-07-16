fn main() {
    assert_eq!(Solution::to_hex(26), "1a");
    assert_eq!(Solution::to_hex(0), "0");
    assert_eq!(Solution::to_hex(-1), "ffffffff");
}

struct Solution;
impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return String::from("0");
        }
        // 转为无符号u32
        let mut u_num = num as u32;
        let mut result = String::new();
        while u_num != 0 {
            let value = u_num & 0xf;
            let digit = if value < 10 {
                char::from_u32('0' as u32 + value)
            } else {
                char::from_u32('a' as u32 + value - 10)
            };
            result.insert(0, digit.unwrap());
            u_num >>= 4;
        }
        result
    }
}