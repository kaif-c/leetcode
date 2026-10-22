mod sol;
use crate::sol::Solution;
use std::process::ExitCode;

struct Case {
    pub i: &'static str,
    pub o: i32,
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i: "ccbbcc", o: 2},
        Case {i: "bbba", o: 2},
        Case {i: "eea", o: 2},
        Case {i: "abcabcbb", o: 3},
        Case {i: "bbbbb", o: 1},
        Case {i: "pwwkew", o: 3},
    ];
    let mut errors = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::length_of_longest_substring(case.i.to_string());
        if out != case.o {
            eprintln!("[ERROR]: Expected {}, but got {}", case.o, out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
