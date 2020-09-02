impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut memo = HashMap::new();
        Solution::dp(s.as_bytes(), 0, p.as_bytes(), 0)
    }

    fn dp(s: &[u8], i: usize, p: &[u8], mut j: usize) -> bool {
        if j == p.len() {
            return i == s.len();
        }
        if i == s.len() {
            if (p.len() - j) % 2 == 1 {
                return false;
            }
            while j + 1 < p.len() {
                if p[j + 1] != '*' as u8 {
                    return false;
                }
                j += 2
            }
            return true;
        }
        if s[i] == p[j] || p[j] == '.' as u8 {
            if j < p.len() - 1 && p[j + 1] == '*' as u8 {
                Solution::dp(s, i, p, j + 2) || Solution::dp(s, i + 1, p, j)
            } else {
                Solution::dp(s, i + 1, p, j + 1)
            }
        } else {
            if j < p.len() - 1 && p[j + 1] == '*' as u8 {
                Solution::dp(s, i, p, j + 2)
            } else {
                false
            }
        }
    }
}
