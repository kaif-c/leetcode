mod row_by_row;
use crate::row_by_row::Solution;
use std::process::ExitCode;

struct Case {
    pub i1: &'static str,
    pub i2: i32,
    pub o: &'static str,
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i1: "PAYPALISHIRING", i2: 3, o: "PAHNAPLSIIGYIR"},
        Case {i1: "PAYPALISHIRING", i2: 4, o: "PINALSIGYAHRPI" },
        Case {i1: "A",              i2: 2, o: "A" },
        Case {i1: "AB",             i2: 1, o: "AB" },
    ];
    let mut errors = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::convert(case.i1.to_string(), case.i2);
        if out != case.o.to_string() {
            eprintln!("[ERROR]: Expected {}, but got {}", case.o, out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
