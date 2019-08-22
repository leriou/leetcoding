
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut pos = 0;
        for i in (0..nums.len()) {
            println!("{}",i);
             if (nums[n] != 0) {
                 let mut tmp = nums[n];
                 nums.insert(i, nums[pos]);
                 nums.insert(pos++, tmp);
             }
        }     
    }
}