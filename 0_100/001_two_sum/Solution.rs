use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let empty_vec = vec![];
        if nums.len() >= 2 {
            let mut maps: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
            for (n, item) in nums.iter().enumerate() {
                let s:i32 = target - item;
                if let Some(pos) = maps.get(&s) {
                    return vec![maps[&s] as i32, n as i32];
                }
                maps.insert(*item, n);
            }   
        }
        empty_vec
    }
}
