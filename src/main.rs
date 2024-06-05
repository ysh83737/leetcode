fn main() {
    assert_eq!(Solution::max_count(3, 3, vec![vec![2,2],vec![3,3]]), 4);
    assert_eq!(Solution::max_count(3, 3, vec![vec![2,2],vec![3,3],vec![3,3],vec![3,3],vec![2,2],vec![3,3],vec![3,3],vec![3,3],vec![2,2],vec![3,3],vec![3,3],vec![3,3]]), 4);
    assert_eq!(Solution::max_count(3, 3, vec![]), 9);
    assert_eq!(Solution::max_count(40000, 40000, vec![]), 40000 * 40000);
    assert_eq!(Solution::max_count(40000, 40000, vec![vec![3000, 3000]]), 3000 * 3000);
    assert_eq!(Solution::max_count(40000, 40000, vec![vec![30000, 30000]]), 30000 * 30000);
}

struct Solution;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.len() == 0 {
            return m * n;
        }

        let mut x = ops[0][0];
        let mut y = ops[0][1];
        for i in 1..ops.len() {
            let a = ops[i][0];
            let b = ops[i][1];
            x = x.min(a);
            y = y.min(b);
        }

        x * y
    }
}
