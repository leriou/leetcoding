class Solution {
    public boolean isSubsequence(String s, String t) {
        int lt = t.length(), ls = s.length(); 
        int l = 0,r = 0;
        while (l < ls && r < lt) {
            if (s.charAt(l) == t.charAt(r)) {
                ++l;
            } 
            ++r;
        }
        return l == ls;
    }
}
