pub struct Solution;

impl Solution {
    // Space = O(1)
    // Time = O(N)
    pub fn climb_stairs(n: i32) -> i32 {
        // INFO:
        // The output is just Fibonacci
        // It just defined Fibonacci sequence
        // So F(N) = F(N - 2) + F(N - 1)
        // where F(1) = 0, F(2) = 1
        // I don't want recursion, i will just use loop
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let c = a + b;
            a = b;
            b = c;
        }
        return b;
    }
}
