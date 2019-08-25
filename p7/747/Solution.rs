impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        for (i, n) in nums.iter().enumerate() {
            let mut flag = true;
            for (k, m) in nums.iter().enumerate() {
                if i != k && *n < *m << 1 {
                    flag = false;
                    break;
                }
            }
            if flag {
                return i as i32;
            }
        }
        return -1;
    }
}