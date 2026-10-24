mod sol;
use crate::sol::Solution;
use std::process::ExitCode;

struct Case {
    pub i: i32,
    pub o: i32,
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i:  123, o:  321},
        Case {i: -123, o: -321},
        Case {i:  120, o:  21},
        Case {i:  1534236469, o:  0},
        Case {i:  -2147483412, o:  -2143847412},
    ];
    let mut errors = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::reverse(case.i);
        if out != case.o {
            eprintln!("[ERROR]: Expected {}, but got {}", case.o, out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
