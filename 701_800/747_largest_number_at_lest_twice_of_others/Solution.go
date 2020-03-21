func dominantIndex(nums []int) int {
    max := -1
    sec := -1
    idx := -1
    for i,j := range nums {
        if j > max {
            sec = max
            max = j
            idx = i
        } else if j > sec  {
            sec = j
        }
    }
    if max - sec >= sec  {
        return idx
    } 
    return -1
}