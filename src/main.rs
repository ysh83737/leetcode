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
        enum Prev { Nothing, Planted, Flowerbed }
        let mut prev = Prev::Nothing;
        let mut count = 0;
        for item in flowerbed {
            if item == 0 {
                match prev {
                    Prev::Nothing => {
                        count += 1;
                        prev = Prev::Planted;
                    },
                    _ => { prev = Prev::Nothing; }
                }
            } else {
                match prev {
                    Prev::Planted => count -= 1,
                    _ => {}
                }
                prev = Prev::Flowerbed;
            }
        }
        count >= n
    }
}
