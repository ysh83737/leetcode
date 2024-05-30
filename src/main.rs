fn main() {
    assert_eq!(Solution::max_count(3, 3, vec![vec![2,2],vec![3,3]]), 4);
    assert_eq!(Solution::max_count(3, 3, vec![vec![2,2],vec![3,3],vec![3,3],vec![3,3],vec![2,2],vec![3,3],vec![3,3],vec![3,3],vec![2,2],vec![3,3],vec![3,3],vec![3,3]]), 4);
}

struct Solution;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut mat = vec![vec![0; n as usize]; m as usize];

        ops.iter().for_each(|x| {
            let a = x[0] as usize;
            let b = x[1] as usize;
            for x in 0..a {
                let row = &mut mat[x];
                for y in 0..b {
                    row[y] += 1;
                }
            }
        });

        let mut result = 0;
        let mut max = 0;
        mat.iter().for_each(|row| {
            row.iter().for_each(|&value| {
                if value > max {
                    max = value;
                    result = 1;
                } else if value == max {
                    result += 1;
                }
            });
        });

        result
    }
}
