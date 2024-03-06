fn main() {
    println!("{:?}", Solution::get_row(30));
}

struct Solution {}
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i64> {
        let row_index = row_index as i64;
        let mut result: Vec<i64> = vec![0; (row_index + 1) as usize];
        result[0] = 1;
        for idx in 1..=(row_index) {
            result[idx as usize] = result[(idx - 1) as usize] * (row_index - idx + 1) / idx;
        }
        result
    }
}
