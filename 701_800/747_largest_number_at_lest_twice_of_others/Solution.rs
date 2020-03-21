impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max = -1;
        let mut sec = -1;
        let mut idx: i32 = -1;
        for (i, n) in nums.iter().enumerate() {
            if *n > max {
                sec = max;
                max = *n;
                idx = i as i32;
            } else if *n > sec {
                sec = *n;
            }
        }
         if max - sec >= sec {
            return idx
        } 
        -1
    }
}