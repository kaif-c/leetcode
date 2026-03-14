
// IDEA: 
// Can't use strings
// What if i reverse the number manually?
// Base 10
// Time: O(log(x)^2)

pub struct Solution {}
struct SigNum {
    num: i32,
    significance: i32,
    order: i32
}

impl SigNum {
    // Gets most significant number and its significance
    // 153 -> 1, 100
    // 53  -> 5, 10
    // 3   -> 3, 1
    fn get_most_significant(mut x: i32) -> SigNum {
        let mut i: i32 = 1;
        let mut order: i32 = 0;
        while x > 9 {
            x /= 10;
            i *= 10;
            order += 1;
        }
        return SigNum { num: x, significance: i, order: order };
    }
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // If its negative its never a palindrome
        if x < 0 {
            return false;
        }

        // 0-9 is always a palindrome
        if x <= 9 {
            return true;
        }

        let mut it_x = x;
        let mut constituents: Vec<i32> = Vec::new();
        let mut orders: Vec<i32> = Vec::new();

        // Loop over the it_x
        // Take away the most significant number and push it to the vector
        while it_x > 0 {
            let sig = SigNum::get_most_significant(it_x);
            orders.push(sig.order);
            
            if orders.len() > 1 {
                let mut cur_ord = orders[orders.len() - 1] + 1;
                Self::fix_order(&mut cur_ord, orders[orders.len() - 2],
                    &mut constituents);
            }
            constituents.push(sig.num);
            it_x -= sig.num * sig.significance;
        }

        let mut cur_ord = 0;
        Self::fix_order(&mut cur_ord, orders[orders.len() - 1],
            &mut constituents);

        // Check if the vector is same forward and backward
        for ind in 0..constituents.len() {
            let rev_ind = constituents.len() - ind - 1;
            // If they are the same then its a palindrome
            // as we are in the centre
            if ind == rev_ind {
                return true;
            }

            // Check if it is a palindrome
            if constituents[ind] != constituents[rev_ind] {
                return false;
            }
        }

        // If even number of numbers then ind is never equal to rev_ind
        // in that case it has to be true
        true
    }

    // Fix the order issue when we encounter a 0
    fn fix_order(cur_ord: &mut i32, last_order: i32,
        constituents: &mut Vec<i32>) {
        while last_order > *cur_ord {
            constituents.push(0);
            *cur_ord += 1;
        }
    }
}
