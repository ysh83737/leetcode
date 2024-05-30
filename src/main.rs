fn main() {
    assert_eq!(Solution::top_k_frequent(
        vec!["i", "love", "leetcode", "i", "love", "coding"].iter().map(|x| { x.to_string() }).collect(),
        2
    ), vec!["i", "love"]);
    assert_eq!(Solution::top_k_frequent(
        vec!["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"].iter().map(|x| { x.to_string() }).collect(),
        4
    ), ["the", "is", "sunny", "day"]);
}

struct Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut word_map = std::collections::HashMap::new();
        words.into_iter().for_each(|word| {
            word_map.entry(word).and_modify(|e| *e += 1).or_insert(1);
        });

        let mut entries: Vec<_> = word_map.into_iter().collect();
        entries.sort_by(|(word1, count1), (word2, count2)| {
            count2.cmp(count1).then(word1.cmp(word2))
        });
        entries.into_iter().take(k as usize).map(|x| x.0).collect()
    }
}
