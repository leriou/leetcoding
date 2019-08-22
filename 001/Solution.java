class Solution {
    public int[] twoSum(int[] nums, int target) {
        Map<Integer, Integer> maps = new HashMap(nums.length);
        for (int i = 0, l = nums.length ; i < l; i++) {
            int s = target - nums[i];
            if (maps.containsKey(s)) {
                return new int[] {i, maps.get(s)};
            }
            maps.put(nums[i], i);
        }
        throw new IllegalArgumentException("No two sum solution");
    }
}