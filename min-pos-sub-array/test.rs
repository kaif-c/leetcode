mod loops;
use crate::loops::Solution;
use std::process::ExitCode;
use std::vec::Vec;

struct Case {
    i1: Vec<i32>,
    i2: i32,
    i3: i32,
    o: i32,
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i1: vec![3, -2, 1, 4],  i2: 2, i3: 3, o: 1},
        Case {i1: vec![-2, 2, -3, 1], i2: 2, i3: 3, o: -1},
        Case {i1: vec![1, 2, 3, 4], i2: 2, i3: 4, o: 3},
        Case {i1: vec![7, 3], i2: 2, i3: 2, o: 10},
        Case {i1: vec![-12, 8], i2: 1, i3: 1, o: 8},
        Case {i1: vec![-1,23,12], i2: 1, i3: 1, o: 12},
    ];
    let mut errors = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::minimum_sum_subarray(case.i1, case.i2, case.i3);
        if out != case.o {
            eprintln!("[ERROR]: Expected {}, but got {}", case.o, out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
