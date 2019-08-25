impl Solution {
  pub fn my_sqrt(x: i32) -> i32 {
        if x <= 0 {return 0;}
        let d = 0.1;
        let dx = x as f64;
        let mut r = dx;
        while (dx - r * r).abs() > d {
            r = (dx / r + r) / 2.0;
        }
        return r as i32;
    }
}