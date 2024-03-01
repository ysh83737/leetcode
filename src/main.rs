fn main() {
    assert_eq!(Solution::plus_one(vec![1,2,3]), vec![1,2,4]);
    assert_eq!(Solution::plus_one(vec![4,3,2,1]), vec![4,3,2,2]);
    assert_eq!(Solution::plus_one(vec![0]), vec![1]);
    assert_eq!(Solution::plus_one(vec![9]), vec![1,0]);
}

struct Solution {}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut output = vec![];
        let mut index = (digits.len() - 1) as i32;
        let mut add_num = 1;

        while index >= 0 {
            let result = digits[index as usize] + add_num;
            if result < 10 {
                output.insert(0, result);
                return [&digits[0..index as usize], &output].concat();
            } else {
                output.insert(0, result - 10);
                add_num = 1;
            }
            index -= 1;
        }
        if add_num > 0 {
            output.insert(0, add_num);
        }
        output
    }
}
