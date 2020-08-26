use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut m: HashMap<i32, i32> = HashMap::new();
        let mut s: Vec<i32> = vec![0; nums2.len()];
        for i in (0..nums2.len()).rev() {
            while s.len() != 0 && s[s.len() - 1] <= nums2[i] {
                s.pop();
            }
            let t = if s.len() == 0 { -1 } else { s[s.len() - 1] };
            m.insert(nums2[i], t);
            s.push(nums2[i]);
        }
        nums1.iter().map(|x| *m.get(x).unwrap()).collect::<Vec<_>>()
    }
}