pub struct Solution;

impl Solution {
    // Time = O(n)
    // Space = O(1)
    pub fn my_atoi(s: String) -> i32 {
        // INFO:
        // First find if there is a -
        // Simply loop over the string
        // Store in i64 instead and check if within i32
        
        if s.len() == 0 {
            return 0;
        }

        let bytes = s.as_bytes();
        let mut iter = bytes.into_iter();
        let mut first = 0i32;

        // Trim the spaces
        for i in 0..bytes.len() {
            first = i as i32;
            if bytes[i] != ' ' as u8 {
                break;
            }
        }

        // Setup if its negative
        let mut is_neg = false;
        if bytes[first as usize] == '-' as u8 {
            is_neg = true;
        }
        if is_neg || bytes[first as usize] == '+' as u8 {
            first += 1;
        }

        // Advance
        if first > 0 {
            iter.nth((first - 1) as usize);
        }

        let mut out = 0i64;
        for c in iter {
            let cur = *c as u8;
            if cur > '9' as u8 || cur < '0' as u8 {
                break;
            }
            out = (out * 10) + (((cur as i8) - ('0' as i8)) as i64);

            // Range checks
            if is_neg {
                if out > ((i32::MIN as i64) * -1) {
                    return i32::MIN;
                }
            }
            else {
                if out > (i32::MAX as i64) {
                    return i32::MAX;
                }
            }

        }

        if is_neg {
            return -out as i32;
        }

        return out as i32;
    }
}
