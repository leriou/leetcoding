class Solution {
    public int lengthOfLongestSubstring(String s) {
        if (s == null || s.isEmpty()) return 0;
        Map<Character, Integer> map = new HashMap<>();
        int i = 0, max = Integer.MIN_VALUE;
        for (int j = 0, l = s.length(); j < l; j++) {
            if (map.keySet().contains(s.charAt(j))) {
                i = Math.max(map.get(s.charAt(j)) + 1, i);
            }
            map.put(s.charAt(j), j);
            max = Math.max(max, (j - i) + 1);
        }
        return max;
    }
}