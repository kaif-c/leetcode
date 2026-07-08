pub struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        // _digits is never [] so we dont need to check that
        // we can just loop from right to left, increment if < 9
        // if > 9 then we set to 0 and increment next value
        let len = digits.len() - 1;
        Self::increment_at(&mut digits, len);
        return digits;
    }

    fn increment_at(digits: &mut Vec<i32>, ind: usize) {
        if digits[ind] == 9 {
            // Carry
            digits[ind] = 0;
            if ind == 0 {
                // If list not big enough
                digits.insert(0, 1);
                return;
            }

            Self::increment_at(digits, ind - 1);
            return;
        }

        digits[ind] += 1;
    }
}
