class Solution {
public:
    int nthUglyNumber(int n) {
        if (n < 1) return 0;
        int* uglyNumbers = new int[n];
        uglyNumbers[0] = 1;
        int next = 1;
        
        int *p2 = uglyNumbers;
        int *p3 = uglyNumbers;
        int *p5 = uglyNumbers;
        
        for (;next < n;next++) {
            int min1 = *p2 * 2 < *p3 * 3 ? *p2 * 2 : *p3 * 3;
            int minUgly = min1 < *p5 * 5 ? min1 : *p5 * 5; 
            uglyNumbers[next] = minUgly;
            
            if (*p2*2 <= uglyNumbers[next]) *p2++;
            if (*p3*3 <= uglyNumbers[next]) *p3++;
            if (*p5*5 <= uglyNumbers[next]) *p5++;
    
        }
        
        return uglyNumbers[next - 1];
    }
    
};
