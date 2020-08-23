class Solution {
  public String[] trulyMostPopular(String[] names, String[] synonyms) {
    HashMap<String, Integer> m = new HashMap<>(names.length);
    HashMap<String, String> w = new HashMap<>(synonyms.length);
    for (String s : names) {
      int i1 = s.indexOf("(");
      m.put(s.substring(0, i1), Integer.parseInt(s.substring(i1 + 1, s.length() - 1)));
    }
    for (String s : synonyms) {
      int i = s.indexOf(",");
      String n1 = s.substring(1, i);
      String n2 = s.substring(i + 1, s.length() - 1);
      while (w.containsKey(n1)) n1 = w.get(n1);
      while (w.containsKey(n2)) n2 = w.get(n2);
      if (!n1.equals(n2)) {
        String nick, truly;
        if (n1.compareTo(n2) > 0) {
          nick = n1;truly = n2;
        } else {
          nick = n2;truly = n1;
        }
        w.put(nick, truly);
        Integer freq = m.getOrDefault(nick, 0) + m.getOrDefault(truly, 0);
        m.put(truly, freq);
        m.remove(nick);
      }
    }
    String[] ans = new String[m.size()];
    int idx = 0;
    for (String e: m.keySet()) {
      StringBuilder sb = new StringBuilder(e);
      sb.append("(");
      sb.append(m.get(e));
      sb.append(")");
      ans[idx++] = sb.toString();
    }
    return ans;
  }
}