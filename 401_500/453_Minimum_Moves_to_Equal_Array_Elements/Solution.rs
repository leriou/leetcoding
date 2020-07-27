impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut nums_copy = nums.clone();
        nums_copy.sort();
        let mut count = 0;
        for i in (1..=nums.len() - 1).rev() {
            count += nums_copy[i] - nums_copy[0];
        }
        count
    }
}
