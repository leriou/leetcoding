impl Solution {
    pub fn range_bitwise_and(mut m: i32,mut n: i32) -> i32 {
        let mut i = 0_usize;
        while m < n {
            m >>= 1;
            n >>= 1;
            i += 1;
        }
        return m << i;
    }
}