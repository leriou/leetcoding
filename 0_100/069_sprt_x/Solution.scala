object Solution {
    val delta = 0.1d
    def mySqrt(x: Int): Int = {
      if (x <= 0) return 0
      val dx = x.toDouble
      var r = dx
      while ( {
        Math.abs(dx - r * r) > delta
      }) r = (dx / r + r) / 2.0d
      r.toInt
    }
}