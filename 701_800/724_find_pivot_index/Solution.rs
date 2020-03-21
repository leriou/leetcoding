impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        for n in &nums {
            total += n;
        }
        let mut left_sum = 0;
        for (i,m) in nums.iter().enumerate() {
            if total - m - left_sum == left_sum {
                return i as i32;
            }
            left_sum += m;
        }
        return -1;
    }
}