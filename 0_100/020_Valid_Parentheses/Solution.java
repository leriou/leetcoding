class Solution {
    public boolean isValid(String s) {
        int length = s.length();
        if (length == 0) return true;
        if (length % 2 == 1) return false;
        char[] st = new char[length + 1];
        st[0] = s.charAt(0);
        int pos = 1;
        for (int i = 1; i < length ; i++) {
            char cur = s.charAt(i);
            char stCur = st[pos];
            if (cur == ')') {
                if (st[pos-1] == '(') --pos; else return false;
            } else if (cur == '}') {
                if (st[pos-1] == '{') { --pos; } else { return false;}
            } else if (cur == ']') {
                if (st[pos-1] == '[') { --pos;} else { return false;}
            } else {
                st[pos++] = cur;
            }
        }
        return pos == 0;
    }
}
