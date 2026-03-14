// THOUGHT:
// Remembered about modulus operator
// It gives the remainder
// So i can get least significant number using % 10
pub struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x <= 9 {
            return true;
        }

        let mut x_cpy = x;
        let mut rev_x = 0;

        while x_cpy > 0 {
            rev_x *= 10; // Sets the current significance (0 if first)
            rev_x += x_cpy % 10; // Get least significant base10 num
            x_cpy /= 10;
        }

        rev_x == x
    }
}
