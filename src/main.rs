fn main() {
    assert_eq!(Solution::matrix_reshape(vec![vec![1,2],vec![3,4]], 1, 4), vec![[1,2,3,4]]);
    assert_eq!(Solution::matrix_reshape(vec![vec![1,2],vec![3,4]], 2, 4), vec![[1,2],[3,4]]);
}

struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m: usize = mat.len();
        let n: usize = if let Some(row) = mat.first() {
            row.len()
        } else {
            0
        };
        if m * n != (r * c) as usize {
            return mat;
        }

        let result_c = c as usize;
        let mut result = vec![vec![0; result_c]; r as usize];
        let mut row_index = 0;
        let mut col_index = 0;
        for row in mat {
            for num in row {
                result[row_index][col_index] = num;
                col_index += 1;
                if col_index >= result_c {
                    col_index -= result_c;
                    row_index += 1;
                }
            }
        }
        result
    }
}
