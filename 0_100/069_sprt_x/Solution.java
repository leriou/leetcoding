class Solution {
	public int mySqrt(int x) {
        if (x <= 0) return 0;
        double dx = (double)x;
        double delta = 0.1d;
        double r = dx;
        while(Math.abs(dx - r * r) > delta) {
            r = (dx / r + r) / 2.0d;   
        }
        return (int)r;
	}
}