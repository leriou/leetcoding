class Solution {
    public int[] sortedSquares(int[] A) {
        int left = 0, right = A.length - 1;
        int[] ans = new int[A.length];
        int idx = A.length - 1;
        while (left <= right) {
            int leftSquare = A[left] * A[left];
            int rightSquare = A[right] * A[right];
            if (left == right) {
                ans[idx] = leftSquare;
                break;
            }
            if (leftSquare > rightSquare) {
                left++;
                ans[idx--] = leftSquare;
            } else if (leftSquare == rightSquare) {
                left++;
                right--;
                ans[idx--] = leftSquare;
                ans[idx--] = rightSquare;
            } else {
                right--;
                ans[idx--] = rightSquare;
            }
        }
        return ans;
    }
}
