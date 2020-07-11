impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        if l == 0 {
            return 0;
        }
        if l == 1 {
            return nums[0];
        }
        let (mut i, mut a, mut b, mut tmp) = (2usize, nums[0], nums[0].max(nums[1]), 0i32);
        while i < l {
            tmp = b;
            b = b.max(a + nums[i]);
            a = tmp;
            i += 1;
        }
        return b;
    }
}