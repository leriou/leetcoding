func moveZeroes(nums []int)  {
    pos := 0
    for i,j := range nums {
        if j != 0 {
            nums[pos], nums[i] = nums[i], nums[pos]
            pos++
        }
    }
}