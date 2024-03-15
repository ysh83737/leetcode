fn main() {
    let nums = vec![-2, 0, 3, -5, 2, -1];
    let num_array = NumArray::new(nums);
    assert_eq!(num_array.sum_range(0, 2), 1);
    assert_eq!(num_array.sum_range(2, 5), -1);
    assert_eq!(num_array.sum_range(0, 5), -3);
}

struct NumArray {
    nums: Vec<i32>,
    sums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![];
        let mut temp = 0;
        for num in nums.iter() {
            temp += num;
            sums.push(temp);
        }
        NumArray { nums, sums }
    }
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let nums = &self.nums;
        let sums = &self.sums;
        sums[right as usize] - sums[left as usize] + nums[left as usize]
    }
}
