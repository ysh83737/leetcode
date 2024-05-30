use std::collections::HashMap;

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
        let mut trie = Trie::new();
        words.into_iter().for_each(|word| {
            trie.insert(word);
        });

        let mut entries = trie.root.entries();
        entries.sort_by(|(word1, count1), (word2, count2)| {
            count2.cmp(count1).then(word1.cmp(word2))
        });
        entries.into_iter().take(k as usize).map(|x| x.0).collect()
    }
}

struct TrieNode {
    pub count: i32,
    pub children: HashMap<char, TrieNode>,
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode{
            count: 0,
            children: HashMap::new()
        }
    }
    pub fn entries(&self) -> Vec<(String, i32)> {
        let mut output = vec![];
        self.children.iter().for_each(|(ch, child)| {
            if child.count > 0 {
                output.push((ch.to_string(), child.count));
            }
            if child.children.len() > 0 {
                child.entries().into_iter().for_each(|(sub, count)| {
                    output.push((
                        [ch.to_string(), sub].concat(),
                        count
                    ))
                });
            }
        });
        output
    }
}

struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: TrieNode::new()
        }
    }
    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TrieNode::new());
        }
        node.count += 1;
    }
}
