impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (row, col) = (matrix.len(), matrix[0].len());
        let (mut l,mut r) = (matrix[0][0], matrix[row-1][col-1]);
        while l < r {
            let mid = (l + r) >> 1;
            if Solution::less_than_mid(&matrix, mid) < k {
                l = mid + 1; 
            } else {
                r = mid;
            }
        }
        l
    }
    pub fn less_than_mid(m: &Vec<Vec<i32>>, mid: i32) -> i32 {
        let (row, col) = (m.len(), m[0].len());
        let (mut i, mut j,mut count) = ((row - 1) as i32, 0_usize, 0_i32);
        while i >= 0 && j < col {
            if m[i as usize][j] <= mid {
                count += (i + 1);
                j += 1;
            } else {
                i -= 1; 
            }
        }
        count
    } 
}
