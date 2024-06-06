fn main() {
    assert_eq!(Solution::can_place_flowers(vec![1,0,0,0,1], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![1,0,0,0,1], 2), false);
    assert_eq!(Solution::can_place_flowers(vec![0,0,0,0,1], 2), true);
    assert_eq!(Solution::can_place_flowers(vec![0,0,0,1,0,1,0,0], 2), true);
    assert_eq!(Solution::can_place_flowers(vec![0,0,0,0,1,0,1,0,0], 2), true);
}

struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;
        let mut index = 0;
        for i in 0..flowerbed.len() {
            let curr = flowerbed.get(i).unwrap();
            if curr == &1 { continue; }

            if index + 1 == i { continue; }
            if i > 0 {
                let prev = flowerbed.get(i - 1).unwrap_or(&0);
                if prev == &1 { continue; }
            }

            let next = flowerbed.get(i + 1).unwrap_or(&0);
            if next == &1 { continue; }
            index = i;
            count += 1;
        }
        return count >= n;
    }
}
