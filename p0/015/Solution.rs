impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res_vec: Vec<Vec<i32>> = vec![];
        let len = nums.len();
        if len < 3 {
            return res_vec;
        }
        let mut ms = nums.clone();
        ms.sort();
        for (i, _) in ms.iter().enumerate() {
            if ms[i] > 0 {
                break;
            }
            if i > 0 && ms[i] == ms[i - 1] {
                continue;
            }
            let mut li = i + 1;
            let mut ri = len - 1;
            while li < ri {
                let sum = ms[i] + ms[li] + ms[ri];
                if sum == 0 {
                    res_vec.push(vec![ms[i], ms[li], ms[ri]]);
                    while li < ri && ms[li] == ms[li + 1] {
                        li += 1;
                    }
                    while li < ri && ms[ri] == ms[ri - 1] {
                        ri -= 1;
                    }
                    li += 1;
                    ri -= 1;
                } else if sum < 0 {
                    li += 1;
                } else if sum > 0 {
                    ri -= 1;
                }
            }
        }
        return res_vec;
    }
}
