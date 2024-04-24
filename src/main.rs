fn main() {
    assert_eq!(Solution::island_perimeter(vec![vec![0,1,0,0],vec![1,1,1,0],vec![0,1,0,0],vec![1,1,0,0]]), 16);
}

struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        const LANG: i32 = 1;
        const NOT_LANG: i32 = 0;

        let mut result = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if value == &LANG {
                    result += 4;
                    if i > 0 {
                        let top = grid.get(i - 1).and_then(|top_row| top_row.get(j)).unwrap_or(&NOT_LANG);
                        if top == &LANG {
                            result -= 2;
                        }
                    }
                    if j > 0 {
                        let left = row.get(j - 1).unwrap_or(&NOT_LANG);
                        if left == &LANG {
                            result -= 2;
                        }
                    }
                }
            }
        }
        result
    }
}
