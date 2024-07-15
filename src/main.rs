fn main() {
    assert_eq!(Solution::find_the_difference(String::from("abcd"), String::from("abcde")), 'e');
    assert_eq!(Solution::find_the_difference(String::from(""), String::from("y")), 'y');
    assert_eq!(Solution::find_the_difference(String::from("a"), String::from("aa")), 'a');
}

struct Solution;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut codes_t = 0;
        for ch in t.chars() {
            codes_t += ch as u32;
        }
        for ch in s.chars() {
            codes_t -= ch as u32;
        }
        char::from_u32(codes_t).unwrap()
    }
}