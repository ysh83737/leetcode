fn main() {
    assert_eq!(Solution::find_relative_ranks(vec![5,4,3,2,1]), vec!["Gold Medal","Silver Medal","Bronze Medal","4","5"]);
    assert_eq!(Solution::find_relative_ranks(vec![10,3,8,9,4]), vec!["Gold Medal","5","Bronze Medal","Silver Medal","4"]);
    assert_eq!(Solution::find_relative_ranks(vec![123123,11921,1,0,123]), vec!["Gold Medal","Silver Medal","4","5","Bronze Medal"]);
}

struct Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let medals = ["Gold Medal", "Silver Medal", "Bronze Medal"];
        let mut result = vec![String::new(); score.len()];

        let mut score_indeies: Vec<_> = score.iter().enumerate().collect();
        score_indeies.sort_by(|a, b| b.1.cmp(a.1));

        score_indeies.iter().enumerate().for_each(|(i, (index, _))| {
            result[*index] = medals
                .get(i)
                .unwrap_or(&(i + 1).to_string().as_str())
                .to_string();
        });
        result
    }
}
