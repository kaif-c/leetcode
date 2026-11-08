mod sol;
use crate::sol::Solution;
use std::process::ExitCode;
use std::vec::Vec;

struct Case {
    i: Vec<i32>,
    o: i32,
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i: vec![1,2,3,4,5,6,7,8,9,10], o: 25},
        Case {i: vec![1,8,6,2,5,4,8,3,7], o: 49},
        Case {i: vec![1, 1], o: 1},
        Case {i: vec![1, 2, 1], o: 2},
        Case {i: vec![1, 2, 3, 2], o: 4},
    ];
    let mut errors = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::max_area(case.i);
        if out != case.o {
            eprintln!("[ERROR]: Expected {}, but got {}", case.o, out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
