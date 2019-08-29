impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ri = numbers.len() - 1;
        for (li, n) in numbers.iter().enumerate() {
            let s = target - n;
            while li < ri {
                let rn:i32 = numbers[ri];
                if rn == s {
                    return vec![li as i32 + 1, ri as i32 + 1];
                } else if rn > s {
                    ri -= 1;
                } else {
                    break;
                }
            }
        }
        return vec![];
    }
}
