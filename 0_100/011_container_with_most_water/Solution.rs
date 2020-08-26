impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() == 0 {
            return 0;
        }
        let (mut water, mut l, mut r) = (0, 0, height.len() - 1);
        while l < r {
            let min_bar = height[l].min(height[r]);
            water = water.max((r - l) as i32 * min_bar);
            while min_bar >= height[l] && l < r {
                l += 1;
            }
            while min_bar >= height[r] && l < r {
                r -= 1
            }
        }
        water
    }
}
