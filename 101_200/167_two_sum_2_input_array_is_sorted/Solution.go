func twoSum(numbers []int, target int) []int {
    ridx := len(numbers) - 1  
    idx := 0
    for idx < ridx {
        s := numbers[ridx] + numbers[idx]
        if s == target {
            return []int{idx + 1, ridx + 1}
        } else if s > target {
            ridx--
        } else {
            idx++
        }
    }
    return []int{}
}