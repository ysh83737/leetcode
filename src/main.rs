fn main() {
    // assert_eq!(Solution::add_binary(String::from("11"), String::from("1")), String::from("100"));
    assert_eq!(Solution::add_binary(String::from("1010"), String::from("1011")), String::from("10101"));
}

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();
        let mut next = '0';
        let mut i = 0;
        let mut chars_a = a.chars().rev();
        let mut chars_b = b.chars().rev();
        while i < a.len() || i < b.len() {
            let mut item_a = chars_a.nth(0).unwrap_or('0');
            if next == '1' {
                if item_a == '1' {
                    item_a = '0';
                } else {
                    item_a = '1';
                    next = '0';
                }
            }
            let item_b = chars_b.nth(0).unwrap_or('0');
            if item_a == item_b {
                result.insert(0, '0');
                if item_a == '1' {
                    next = '1';
                }
            } else {
                result.insert(0, '1');
            }
            i += 1;
        }
        if next == '1' {
            result.insert(0, '1');
        }
        result
    }
}
