class Solution {
public:
    int integerBreak(int n) {
        int mmax = 0, res = n % 3, les = n / 3; //求余,求整
        if (n <= 3) mmax = 1 * (n - 1);
        else {
            if (res == 0) mmax = pow(3,les); //不分成2，都分成3
            else if (res == 1) mmax = pow(3, les -1) * 2 *2; //分成2个2，其余为3
            else mmax = pow(3,les) * 2; //分成一个2，其余为3
        }
        return mmax;
    }
};
