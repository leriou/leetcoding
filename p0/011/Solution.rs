use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if  height.len() == 0 {
            return 0;
        }
        let mut water: i32   = 0;
        let mut l            = 0;
        let mut r            = height.len() -1;
        let mut min_bar: i32 = 0;
        
        while l < r {
            min_bar = cmp::min(height[l], height[r]);
            water = cmp::max(water, (r-l) as i32 * min_bar);
            while min_bar >= height[l] && l < r {
                l+=1;
            }
            while min_bar >= height[r] && l < r {
                r-=1
            }
        }
        return water;
    }
}
