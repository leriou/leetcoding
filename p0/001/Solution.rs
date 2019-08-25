use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut maps: HashMap<i32, usize> = HashMap::new();
        for (n, item) in nums.iter().enumerate() {
            let s:i32 = target - item;
            if maps.contains_key(&s) {
                return vec![*maps.get(&s).unwrap() as i32, n as i32];
            }
            maps.insert(*item, n);
        }
        panic!("no two num");
    }
}