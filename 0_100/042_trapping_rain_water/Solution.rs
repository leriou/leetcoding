impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut ans, n) = (0_i32, height.len());
        if n > 0 {
            let (mut left, mut right) = (0, n as i32 - 1);
            let (mut l_max, mut r_max) = (height[0], height[n - 1]);
            while left <= right {
                l_max = l_max.max(height[left as usize]);
                r_max = r_max.max(height[right as usize]);
                if l_max < r_max {
                    ans += l_max - height[left as usize];
                    left += 1;
                } else {
                    ans += r_max - height[right as usize];
                    right -= 1;
                }
            }
        }
        ans
    }
}