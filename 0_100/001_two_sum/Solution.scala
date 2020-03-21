import scala.collection.mutable.Map;

object Solution {
  def twoSum(nums: Array[Int], target: Int): Array[Int] = {
    val maps:Map[Int, Int] = Map()
    var n:Int = nums.length
    while (n > 0) {
      n -= 1
      val i:Int = nums(n)
      val t:Int = target - i
      if (maps.contains(t)) return Array[Int](maps.getOrElse(t, 0) , n)
      maps += (i -> n)
    }
    Array()
  }
}
