func twoSum(nums []int, target int) []int {
	maps := map[int]int{}
	for i := 0; i < len(nums); i++ {
		s := target - nums[i]
		num, ok := maps[s]
		if ok {
			tmp := [...]int{num, i}
			return tmp[:2]
		}
		maps[nums[i]] = i
	}
	return []int{}
}