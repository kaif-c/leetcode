// INFO:
// Simply loop over each characters?
// A hash map of all combinations? too slow for this
// Must do 3 number add instead of just 2
// XOR is just binary add
// string[i] - '0' == int(string[i])

pub struct Solution;
use std::cmp::max;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut out = String::with_capacity(max(a.len(), b.len()));
        let mut a_ind = (a.len() - 1) as i32;
        let mut b_ind = (b.len() - 1) as i32;
        let mut carry = 0u8;
        const A0: u8 = '0' as u8;
        // Strings can be different sizes
        // Adding is just XOR
        // Check for a carry
        while a_ind >= 0 || b_ind >= 0 || carry > 0 {
            let a_val = if a_ind >= 0 {
                a.as_bytes()[a_ind as usize] as u8 - A0
            } else {
                0
            };
            let b_val = if b_ind >= 0 {
                b.as_bytes()[b_ind as usize] as u8 - A0
            } else {
                0
            };

            let o_carry = carry;
            let mut val = a_val^b_val;

            carry = (a_val & b_val) | (o_carry & val);
            val = val^o_carry;

            out.insert(0, (val + A0) as char);

            if a_ind >= 0 {
                a_ind -= 1;
            }
            if b_ind >= 0 {
                b_ind -= 1;
            }
        }

        return out;
    }
}
