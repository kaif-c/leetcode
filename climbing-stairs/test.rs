mod fibonacci;
use crate::fibonacci::Solution;
use std::process::ExitCode;

struct Case {
    pub i: i32,
    pub o: i32,
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i: 2,  o: 2},
        Case {i: 3,  o: 3},
        Case {i: 4,  o: 5},
        Case {i: 5,  o: 8},
        Case {i: 6,  o: 13},
        Case {i: 45, o: 1836311903},
    ];
    let mut errors = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::climb_stairs(case.i);
        if out != case.o {
            eprintln!("[ERROR]: Expected {}, but got {}", case.o, out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
