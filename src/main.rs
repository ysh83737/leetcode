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
        let mut x = m;
        let mut y = n;
        ops.iter().for_each(|op| {
            let a = op[0];
            let b = op[1];
            x = x.min(a);
            y = y.min(b);
        });

        x * y
    }
}
