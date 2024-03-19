fn main() {
    let nums1 = vec![1,2,2,1];
    let nums2 = vec![2,2];
    assert_eq!(Solution::intersect(nums1,nums2), vec![2,2]);
}

struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map1: HashMap<i32, u32> = HashMap::new();
        for num in nums1 {
            map1.entry(num).and_modify(|count| { *count += 1; }).or_insert(1);
        }
        let mut map2: HashMap<i32, u32> = HashMap::new();
        for num in nums2 {
            if let Some(count1) = map1.get(&num) {
                map2.entry(num).and_modify(|count2| {
                    if *count2 < *count1 {
                        *count2 += 1;
                    }
                }).or_insert(1);
            }
        }
        map2.into_iter().flat_map(|(num, count)| vec![num; count as usize]).collect()
    }
}
