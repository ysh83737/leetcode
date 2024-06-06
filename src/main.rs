fn main() {
    assert_eq!(Solution::can_place_flowers(vec![1,0,0,0,1], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![1,0,0,0,1], 2), false);
    assert_eq!(Solution::can_place_flowers(vec![0,0,0,0,1], 2), true);
    assert_eq!(Solution::can_place_flowers(vec![0,0,0,1,0,1,0,0], 2), true);
    assert_eq!(Solution::can_place_flowers(vec![0,0,0,0,1,0,1,0,0], 2), true);
    assert_eq!(Solution::can_place_flowers(vec![1,0,0,0,0,1], 2), false);
    assert_eq!(Solution::can_place_flowers(vec![0], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![1,0], 1), false);
}

struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let m = flowerbed.len() as i32;
        let mut count = 0;
        let mut prev = -1;
        for i in 0..m {
            if flowerbed[i as usize] == 1 {
                let gap = if prev < 0 {
                    i
                } else {
                    i - prev - 2
                };
                count += gap / 2;
                prev = i;
            }
        }
        if prev < 0 {
            count += (m + 1) / 2;
        } else {
            count += (m - prev - 1) / 2;
        }
        count >= n
    }
}
