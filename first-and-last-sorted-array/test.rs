mod sol;
use crate::sol::Solution;
use std::process::ExitCode;
use std::vec::Vec;

struct Case {
    pub i1: Vec<i32>,
    pub i2: i32,
    pub o: Vec<i32>,
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i1: vec![5,7,7,8,8,10], i2: 8, o: vec![3,4]},
        Case {i1: vec![5,7,7,8,8,10], i2: 6, o: vec![-1,-1]},
        Case {i1: vec![],             i2: 0, o: vec![-1,-1]},
        Case {i1: vec![2,2],          i2: 2, o: vec![0,1]},
        Case {i1: vec![2],          i2: 2, o: vec![0,0]},
    ];
    let mut errors = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::search_range(case.i1, case.i2);
        if out != case.o {
            eprintln!("[ERROR]: Expected {:?}, but got {:?}", case.o, out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
