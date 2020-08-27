impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.insert(0, 0);
        heights.push(0);
        let (n, mut ans) = (heights.len(), 0_i32);
        let mut st = vec![];
        for i in 0..n {
            while !st.is_empty() && heights[st[st.len() - 1]] > heights[i] {
                let h = heights[st.pop().unwrap()];
                ans = ans.max((i - st[st.len() - 1] - 1) as i32 * h);
            }
            st.push(i);
        }
        ans
    }
}