mod modulus_method;
use modulus_method::Solution;

const CASES: [(i32, bool); 10] = [
    (4790797, false),
    (554, false),
    (660, false),
    (001, true),
    (456, false),
    (484683, false),
    (11001, false),
    (101, true),
    (1010, false),
    (121, true)
];

pub fn main() {
    for case in CASES {
        if Solution::is_palindrome(case.0) == case.1 {
            println!("SUCCESS");
        }
        else {
            println!("FAIL");
        }
    }
}
