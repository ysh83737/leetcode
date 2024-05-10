fn main() {
    assert_eq!(Solution::find_relative_ranks(vec![5,4,3,2,1]), vec!["Gold Medal","Silver Medal","Bronze Medal","4","5"]);
    assert_eq!(Solution::find_relative_ranks(vec![10,3,8,9,4]), vec!["Gold Medal","5","Bronze Medal","Silver Medal","4"]);
    assert_eq!(Solution::find_relative_ranks(vec![123123,11921,1,0,123]), vec!["Gold Medal","Silver Medal","4","5","Bronze Medal"]);
}

struct Solution;

impl Solution {
    pub fn find_relative_ranks(mut score: Vec<i32>) -> Vec<String> {
        let medals = ["Gold Medal", "Silver Medal", "Bronze Medal"];
        let n = score.len();
        let mut result = vec![String::new(); n];
        for i in 0..n {
            let max_index = Solution::find_max_index(&score);
            result[max_index] = medals.get(i).unwrap_or(&(i + 1).to_string().as_str()).to_string();
            score[max_index] = -1;
        }
        result
    }
    pub fn find_max_index(score: &Vec<i32>) -> usize {
        let mut index = 0;
        for i in 1..score.len() {
            if score[i] > score[index] {
                index = i;
            }
        }
        index
    }
}
