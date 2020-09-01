impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut arr: Vec<i32> = vec![];
        Solution::backtrace(&nums, &mut arr, &mut ans);
        return ans;
    }

    fn backtrace(nums: &Vec<i32>, arr: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if arr.len() == nums.len() {
            ans.push(arr.clone());
        } else {
            for i in nums {
                if !arr.contains(i) {
                    arr.push(*i);
                    Solution::backtrace(nums, arr, ans);
                    arr.pop();
                }
            }
        }
    }
}