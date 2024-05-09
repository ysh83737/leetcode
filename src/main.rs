use std::collections::HashMap;

fn main() {
    assert_eq!(Solution::find_words(vec!["Hello".to_string(),"Alaska".to_string(),"Dad".to_string(),"Peace".to_string()]), vec!["Alaska","Dad"]);
    assert_eq!(Solution::find_words(vec!["omk".to_string()]), vec![] as Vec<String>);
    assert_eq!(Solution::find_words(vec!["Aasdfghjkl".to_string(),"Qwertyuiop".to_string(),"zZxcvbnm".to_string()]), vec!["Aasdfghjkl","Qwertyuiop","zZxcvbnm"]);
}

struct Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        const KEYBOARD: [&str; 3] = [
            "qwertyuiop",
            "asdfghjkl",
            "zxcvbnm"
        ];
        let mut letter_map = HashMap::new();
        KEYBOARD.iter().enumerate().for_each(|(i, row)| {
            for key in row.chars() {
                letter_map.insert(key, i);
            }
        });

        words.into_iter().filter(|word| {
            let mut temp = None;
            for key in word.to_lowercase().chars() {
                let row_index = letter_map.get(&key);
                if temp != None && row_index != temp {
                    return false;
                }
                temp = row_index;
            }
            true
        }).collect()
    }
}
