fn main() {
    assert_eq!(Solution::find_restaurant(
        vec!["Shogun","Tapioca Express","Burger King","KFC"].iter().map(|x| x.to_string()).collect(),
        vec!["Piatti","The Grill at Torrey Pines","Hungry Hunter Steakhouse","Shogun"].iter().map(|x| x.to_string()).collect()
    ), vec!["Shogun"]);
    assert_eq!(Solution::find_restaurant(
            vec!["Shogun", "Tapioca Express", "Burger King", "KFC"].iter().map(|x| x.to_string()).collect(),
            vec!["KFC", "Shogun", "Burger King"].iter().map(|x| x.to_string()).collect()
        ), vec!["Shogun"]);
}

struct Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let (short, long) = if list1.len() < list2.len() {
            (list1, list2)
        } else {
            (list1, list2)
        };
        let mut map1 = std::collections::HashMap::new();
        short.iter().enumerate().for_each(|(i, item)| {
            map1.insert(item, i);
        });
        let mut result = vec![];
        let mut min = short.len() + long.len();
        for (i, item) in long.into_iter().enumerate() {
            if i > min { break; }
            if let Some(value) = map1.get(&item) {
                let sum = value + i;
                if sum == min {
                    result.push(item);
                } else if sum < min {
                    min = sum;
                    result = vec![item];
                }
            }
        };
        result
    }
}
