object Solution {
  def moveZeroes(nums: Array[Int]): Unit = {
    var pos = 0
    var i = 0
    while (i < nums.length) {
      if (nums(i) != 0) {
        val tmp = nums(i)
        nums(i) = nums(pos)
        nums(pos) = tmp
        pos += 1
      }
      i += 1
    }
  }
}