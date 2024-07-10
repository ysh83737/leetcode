fn main() {
    assert_eq!(Solution::add_binary(String::from("11"), String::from("1")), String::from("100"));
    assert_eq!(Solution::add_binary(String::from("1010"), String::from("1011")), String::from("10101"));
}

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut i = a.len();
        let mut j = b.len();
        let mut carry = 0;
        let mut result = String::new();
        let mut unshift = |item: u32| {
            result.insert_str(0, item.to_string().as_str());
        };
        let mut chars_a = a.chars().rev();
        let mut chars_b = b.chars().rev();

        while i > 0 && j > 0 {
            let item_a = chars_a.next().unwrap().to_digit(10).unwrap();
            let item_b = chars_b.next().unwrap().to_digit(10).unwrap();
            let sum = carry + item_a + item_b;
            carry = sum / 2;
            unshift(sum % 2);
            i -= 1;
            j -= 1;
        }
        while i > 0 {
            let item_a = chars_a.next().unwrap().to_digit(10).unwrap();
            let sum = carry + item_a;
            carry = sum / 2;
            unshift(sum % 2);
            i -= 1;
        }
        while j > 0 {
            let item_b = chars_b.next().unwrap().to_digit(10).unwrap();
            let sum = carry + item_b;
            carry = sum / 2;
            unshift(sum % 2);
            j -= 1;
        }
        if carry > 0 {
            unshift(carry);
        }
        result
    }
}
