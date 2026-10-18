pub struct Solution;

impl Solution {
    // Space = O(1)
    // Time = O(log N)
    pub fn my_sqrt(x: i32) -> i32 {
        // INFO:
        // Hint says to explore all values
        // So we won't be using Newton-Raphson method
        // Binary search to check if y*y == x
        // y will always be 0->x

        // 64 bit to be safe from overflow as range only includes i32
        let x64 = x as i64;
        let mut lower = 0i64;
        let mut upper = x64 + 1;
        let mut mid = 0i64;

        // Mid will always fit in i32 so no need to check
        while lower < upper {
            mid = (upper / 2) + (lower / 2);
            if mid * mid > x64 {
                upper = mid;
                continue;
            }

            if mid * mid < x64 {
                // Remove excess checks since we want rounded down value
                if (mid + 1) * (mid + 1) > x64 {
                    return mid as i32;
                }
                lower = mid + 1;
                continue;
            }

            return mid as i32;
        }

        return mid as i32;
    }
}
