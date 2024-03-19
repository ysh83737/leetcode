fn main() {
    let nums = vec![-2, 0, 3, -5, 2, -1];
    let num_array = NumArray::new(nums);
    assert_eq!(num_array.sum_range(0, 2), 1);
    assert_eq!(num_array.sum_range(2, 5), -1);
    assert_eq!(num_array.sum_range(0, 5), -3);
}

struct NumArray {
    sums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![0];
        for (i, num) in nums.iter().enumerate() {
            sums.push(sums[i] + num)
        }
        NumArray { sums }
    }
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sums[(right + 1) as usize] - self.sums[left as usize]
    }
}
