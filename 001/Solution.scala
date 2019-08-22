import scala.collection.mutable.Map;
import scala.Mutable.Int

object Solution {
    def twoSum(nums: Array[Int], target: Int): Array[Int] = {
        val maps:Map[Int, Int] = Map()
    
        val n:Int = 0
        for (i:Int <- nums) {
            val s:Int = target - i
            if (maps.contains(s)) {
                return Array[Int](maps.get(s).getOrElse(0), n)
            }
            maps.put(i, n)
            n += 1
        }
        Array()
    }
}