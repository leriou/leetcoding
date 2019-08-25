import (
    "math"
)
func mySqrt(x int) int {
    if x <= 0 {
      return 0  
    } 
    var d float64 = 0.1
    var dx float64 = float64(x)
    r := dx
    for math.Abs(dx - r*r) > d {
        r = (float64(x) / r + r) / 2.0
    }
    return int(r)
}