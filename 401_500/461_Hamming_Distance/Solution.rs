impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        format!("{:b}", x ^ y)
            .as_bytes()
            .iter()
            .filter_map(|&x| if x == 49 { Some(1) } else { None })
            .count() as i32
    }
}
