object Solution {
    def searchInsert(nums: Array[Int], target: Int): Int = {
        nums.filter(x => x < target)
        .foldLeft(0)((m, n)=> m + 1)
    }
}
