class Solution {
    public int[] intersect(int[] nums1, int[] nums2) {
        if (nums2.length < nums1.length) {
            return intersect(nums2, nums1);
        }
        int[] ans = new int[nums1.length];
        Map<Integer,Integer> m = new HashMap();
        for (int i = 0, l = nums1.length; i < l; i++) {
            m.putIfAbsent(nums1[i], 0);
            int c = m.get(nums1[i]);
            m.put(nums1[i], ++c);
        }
        int idx = 0;
        for (int j = 0, l2 = nums2.length; j < l2; j++) {
            if (m.containsKey(nums2[j])) {
                int count = m.get(nums2[j]);
                if (count > 0) {
                    ans[idx++] = nums2[j]; 
                    m.put(nums2[j], --count);
                }           
            }
        }
        return Arrays.copyOfRange(ans, 0, idx);
    }
}
