impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return n as i32;
        }
        let mut subs = vec![];
        subs.push(nums[0]);
        for i in 1..n {
            let last = subs.last().unwrap();
            if last < &nums[i] {
                subs.push(nums[i]);
            } else {
                let (mut l, mut r) = (0, subs.len() - 1);
                while l < r {
                    let mid = l + (r - l) / 2;
                    if subs[mid] < nums[i] {
                        l = mid + 1;
                    } else {
                        r = mid;
                    }
                }
                subs[l] = nums[i];
            }
        }
        subs.len() as i32
    }
}
