use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut maps: HashMap<i32,i32> = HashMap::new();
        let mut n = 0;
        for item in &nums {
            let s:i32 = target - item;
            if maps.contains_key(&s) {
                return vec![*maps.get(&s).unwrap(), n];
            }
            maps.insert(*item, n);
            n += 1;
        }
        panic!("no two num");
    }    
}