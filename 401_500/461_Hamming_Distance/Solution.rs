impl Solution {
  pub fn hamming_distance(x: i32, y: i32) -> i32 {
      format!("{:b}", x^y).as_bytes()
      .iter().map(|&x| if x == 49 {1} else {0} ).filter(|&x| x == 1)
      .count() as i32
    }
}
