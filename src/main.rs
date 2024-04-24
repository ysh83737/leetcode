fn main() {
    assert_eq!(Solution::find_content_children(vec![1,2,3], vec![1,1]), 1);
    assert_eq!(Solution::find_content_children(vec![1,2], vec![1,2,3]), 2);
    assert_eq!(Solution::find_content_children(vec![1,2,3], vec![1,2,3]), 3);
    assert_eq!(Solution::find_content_children(vec![10,9,8,7,10,9,8,7], vec![10,9,8,7]), 4);
}

struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        let mut result = 0;
        g.sort();
        s.sort();

        let mut i = 0;
        let mut j = 0;
        while i < g.len() && j < s.len() {
            let g_num = g[i];
            let s_num = s[j];
            if s_num >= g_num {
                result += 1;
                i += 1;
            } else {
            }
            j += 1;
        }
        result
    }
}
