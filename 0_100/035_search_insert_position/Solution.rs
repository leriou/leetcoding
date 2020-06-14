impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.iter().filter(|&&x| x < target).fold(0, |acc, x| acc + 1)
    }
}