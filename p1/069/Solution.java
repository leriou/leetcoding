class Solution {
	public int mySqrt(int x) {
        if (x <= 0) return 0;
        double dx = (double)x;
        double r = dx;
        double delta = 0.1d;
        while(Math.abs(dx - r * r) > delta) {
            r = (x/r + r) / 2.0d;   
        }
        return (int)r;
	}
}
