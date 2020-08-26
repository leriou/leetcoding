impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let len = nums.len();
        if len >= 3 {
            nums.sort_unstable();
            for i in 0..len {
                if nums[i] > 0 {
                    break;
                }
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }
                let (mut li, mut ri) = (i + 1, len - 1);
                while li < ri {
                    let sum = nums[i] + nums[li] + nums[ri];
                    if sum == 0 {
                        ans.push(vec![nums[i], nums[li], nums[ri]]);
                        while li < ri && nums[li] == nums[li + 1] {
                            li += 1;
                        }
                        while li < ri && nums[ri] == nums[ri - 1] {
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
        }
        ans
    }
}