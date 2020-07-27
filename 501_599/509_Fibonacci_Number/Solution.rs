impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let nu = n as usize;
        let mut f = vec![0i32; nu + 1];
        f[1] = 1;
        for i in 2..=nu {
            f[i] = f[i - 1] + f[i - 2];
        }
        return f[nu];
    }
}
