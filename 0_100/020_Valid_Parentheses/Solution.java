class Solution {
  public boolean isValid(String s) {
        int length = s.length();
        if (length == 0) return true;
        if (length % 2 == 1) return false;
        Character[] st = new Character[length + 1];
        Map<Character, Character> m = new HashMap() {{
            put(')','(');
            put('}','{');
            put(']','[');
        }};
        st[0] = Character.valueOf(s.charAt(0));
        int pos = 1;
        for (int i = 1; i < length ; i++) {
            Character cur = Character.valueOf(s.charAt(i));
            if (m.containsKey(cur)) {
                if (m.get(cur).equals(st[pos-1])) --pos; else return false;
            } else {
                st[pos++] = cur;
            }
        }
        return pos == 0;
    }
}


