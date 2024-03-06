fn main() {
    assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 5);
}

struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut j = 0;
        let mut result = 0;
        for i in 1..prices.len() {
            let profit = prices[i] - prices[j];
            if profit > result {
                result = profit;
            } else if profit < 0 {
                j = i;
            }
        }
        result
    }
}
