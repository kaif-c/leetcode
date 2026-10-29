mod sol;
use crate::sol::Solution;
use std::process::ExitCode;

struct Case {
    pub i: String,
    pub o: i32
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i: "42".to_string(),       o: 42},
        Case {i: "-042".to_string(),     o: -42},
        Case {i: "1337c0d3".to_string(), o: 1337},
        Case {i: "0-1".to_string(),      o: 0},
        Case {i: "   -042".to_string(),  o: -42},
        Case {i: "9223372036854775808".to_string(),  o: 2147483647},
        Case {i: "-91283472332".to_string(),  o: -2147483648},
    ];
    let mut errors = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::my_atoi(case.i);
        if out != case.o {
            eprintln!("[ERROR]: Expected {}, but got {}", case.o, out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
