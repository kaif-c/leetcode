pub struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut out = 0i32;
        while x != 0 {
            if let Some(v) = out.checked_mul(10) {
                out = v;
            }
            else {
                return 0;
            }
            if let Some(v) = out.checked_add(x % 10) {
                out = v;
            }
            else {
                return 0;
            }
            x /= 10;
        }
        return out;
    }
}
