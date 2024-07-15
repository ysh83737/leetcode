fn main() {
    assert_eq!(Solution::find_the_difference(String::from("abcd"), String::from("abcde")), 'e');
    assert_eq!(Solution::find_the_difference(String::from(""), String::from("y")), 'y');
    assert_eq!(Solution::find_the_difference(String::from("a"), String::from("aa")), 'a');
}

struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut map: HashMap<char, u32> = HashMap::new();
        for ch in s.chars() {
            map.entry(ch).and_modify(|e| *e += 1).or_insert(1);
        }
        for ch in t.chars() {
            if let Some(count) = map.get_mut(&ch) {
                if count == &0 {
                    return ch;
                }
                *count -= 1;
            } else {
                return ch;
            }
        }
        unreachable!()
    }
}