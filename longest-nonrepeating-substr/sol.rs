pub struct Solution;
use std::cmp::min;

impl Solution {
    // Time = O(n)
    // Space = O(1)
    pub fn length_of_longest_substring(s: String) -> i32 {
        // INFO:
        // only English letters, symbols, & spaces so fits within ascii
        // Store a ascii int table to show position of a char previously or not
        // Another number for max non-repeating char
        // Another number for cur non-repeating char, incremented when uncommon char, compared when
        // common

        let mut table = [-1i32; 127];
        let mut cur_len = 0i32;
        let mut max_len = 0i32;
        for (i, c) in s.as_bytes().iter().enumerate() {
            if table[*c as usize] != -1 {
                if cur_len > max_len {
                    max_len = cur_len;
                }
                // We calculate the minimum for the case
                // C C B B C C
                //     ^-^ will be forgotten if
                //   ^-----^ is hit
                cur_len = min(i as i32 - (table[*c as usize] + 1), cur_len);
            }
            // Unique
            table[*c as usize] = i as i32;
            cur_len += 1;
        }
        if cur_len > max_len {
            return cur_len;
        }
        return max_len;
    }
}
