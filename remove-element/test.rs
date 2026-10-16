mod two_pointer_loop;
use crate::two_pointer_loop::Solution;
use std::process::ExitCode;
use std::vec::Vec;

struct Case {
    pub i1: Vec<i32>,
    pub i2: i32,
    pub o: Vec<i32>,
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i1: vec![3, 2, 2, 3], i2: 3, o: vec![2, 2]},
        Case {i1: vec![0, 1, 2, 2, 3, 0, 4, 2], i2: 2, o: vec![0, 1, 3, 0, 4]},
    ];

    let mut exit_code = ExitCode::SUCCESS;
    for case in cases {
        let mut inp = case.i1.clone();
        let k = Solution::remove_element(&mut inp, case.i2) as usize;
        let mut error = false;

        if k != case.o.len() {
            eprintln!("K value not same");
            error = true;
        }

        for i in 0..case.o.len() {
            if case.o[i] != inp[i] {
                eprintln!("Expected: {:?}, but got {inp:?}", case.o);
                error = true;
                break;
            }
        }

        if error {
            exit_code = ExitCode::FAILURE;
        }
        else {
            println!("SUCCESS");
        }
    }
    return exit_code;
}
