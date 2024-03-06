fn main() {
    let result = Solution::generate(5);
    println!("{:?}", result);
}

struct Solution {}
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        for idx in 0..(num_rows as usize) {
            let mut curr = vec![1];
            if idx > 0 {
                let prev = &result[idx - 1];
                for i in 1..prev.len() {
                    curr.push(prev[i - 1] + prev[i]);
                }
                curr.push(1);
            }
            result.push(curr);
        }
        result
    }
}
