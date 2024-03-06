fn main() {
    println!("{:?}", Solution::get_row(3));
}

struct Solution {}
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![1];
        for row in 1..=(row_index as usize) {
            result.push(1);
            for idx in (1..row).rev() {
                result[idx] += result[idx - 1];
            }
            println!("{:?}", result);
        }
        result
    }
}
