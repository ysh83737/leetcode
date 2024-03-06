fn main() {
    assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 5);
}

struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut min_price = i32::MAX;
        for i in 0..prices.len() {
            let price = prices[i];
            if price < min_price {
                min_price = price;
            } else if (price - min_price) > result {
                result = price - min_price;
            }
        }
        result
    }
}
