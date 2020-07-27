impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let (mut pre, mut cur) = (0, 1);
        for i in 2..=n {
            let t = cur + pre;
            pre = cur;
            cur = t;
        }
        return cur;
    }
}
