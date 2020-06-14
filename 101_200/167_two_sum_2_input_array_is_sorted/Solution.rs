impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = numbers.len() - 1;
        for (l, n) in numbers.iter().enumerate() {
            let s = target - n;
            while l < i {
                let pos:i32 = numbers[i];
                if pos == s {
                    return vec![l as i32 + 1, i as i32 + 1];
                } else if pos > s {
                    i -= 1;
                } else {
                    break;
                }
            }
        }
        return vec![];
    }
}