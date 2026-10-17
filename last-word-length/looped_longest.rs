pub struct Solution;

impl Solution {
    // Space = O(1)
    // Time = O(n)
    pub fn length_of_last_word(s: String) -> i32 {
        let mut len = 0;
        let mut was_space = false;
        // O(n) loop
        // UTF8 safe version even tho not required here
        for c in s.chars() {
            if c == ' ' {
                was_space = true;
            }
            else {
                if was_space {
                    len = 1;
                    was_space = false;
                }
                else {
                    len += 1;
                }
            }
        }
        return len;
    }
}
