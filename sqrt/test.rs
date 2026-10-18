mod binary_search;
use crate::binary_search::Solution;
use std::process::ExitCode;

struct Case {
    pub i: i32,
    pub o: i32,
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i: 4, o: 2},
        Case {i: 8, o: 2},
        Case {i: 10, o: 3},
        Case {i: 2147395599, o: 46339},
    ];
    let mut errors: ExitCode = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::my_sqrt(case.i);
        if out != case.o {
            eprintln!("[ERROR]: Expected {}, but got {}", case.o, out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
