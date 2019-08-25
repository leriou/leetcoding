func twoSum(numbers []int, target int) []int {
    ridx := len(numbers) - 1  
    for idx, n := range numbers {
        s := target - n
        for ridx > idx {
            rn := numbers[ridx]  
            if s == rn {
                return []int{idx + 1, ridx + 1} 
            } else if s < rn {
                ridx--
            } else {
                break;
            }
        }
    }
    return []int{}
}