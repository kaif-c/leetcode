mod solution;
use crate::solution::Solution;
use std::process::ExitCode;
use std::vec::Vec;

struct Case {
    pub i: Vec<i32>,
    pub o: i32,
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i: vec![2,7,1,4],   o: 5},
        Case {i: vec![7,1,5,3,6,4],  o: 5},
        Case {i: vec![7,6,4,3,1],   o: 0},
        Case {i: vec![1,2],   o: 1},
        Case {i: vec![2, 3, 1], o: 1},
    ];
    let mut errors = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::max_profit(case.i);
        if out != case.o {
            eprintln!("[ERROR]: Expected {}, but got {}", case.o, out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
