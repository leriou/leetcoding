class Solution {
    public int[] twoSum(int[] numbers, int target) {
        int ri = numbers.length - 1;
        for (int i = 0; i < numbers.length ; i++) {
            int s = target - numbers[i];
            while (i < ri) {
                if (s == numbers[ri]) {
                    return new int[]{i+1, ri +1};
                } else if ( s < numbers[ri]) {
                    ri--;
                } else {
                    break;
                }
            }
        }
        return new int[] {};
    }
}