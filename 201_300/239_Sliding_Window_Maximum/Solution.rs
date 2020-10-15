impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let (ku, n) = (k as usize, nums.len());
        if ku * n == 0 {
            return vec![];
        }
        if k == 1 {
            return nums;
        }
        let (mut left, mut right) = (vec![0_i32; n], vec![0_i32; n]);
        left[0] = nums[0];
        right[n - 1] = nums[n - 1];

        for i in 1..n {
            left[i] = if i % ku == 0 {
                nums[i]
            } else {
                nums[i].max(left[i - 1])
            };

            let j = n - i - 1;
            right[j] = if (j + 1) % ku == 0 {
                nums[j]
            } else {
                nums[j].max(right[j + 1])
            };
        }

        let mut ans: Vec<i32> = vec![];
        for i in 0..(n - ku + 1) {
            ans.push(right[i].max(left[i + ku - 1]));
        }
        ans
    }
}
