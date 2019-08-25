impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ri: i32 = numbers.len() as i32 - 1;
        let mut li: i32 = 0;
        for n in &numbers {
            let s = target - n;
            while li < ri {
                let rn:i32 = *numbers.get(ri as usize).unwrap();
                if rn == s {
                    return vec![li + 1, ri +1];
                } else if rn > s {
                    ri -= 1;
                } else {
                    break;
                }
            }
            li += 1;
        }
        return vec![];
    }
}