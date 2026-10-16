mod looped_modulus;
use crate::looped_modulus::Solution;
use std::process::ExitCode;

struct Case {
    pub i: [&'static str; 2],
    pub o: &'static str
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i: ["1", "0"], o: "1"},
        Case {i: ["1", "1"], o: "10"},
        Case {i: ["101", "11"], o: "1000"},
        Case {i: ["10", "0"], o: "10"}
    ];
    let mut errors: ExitCode = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::add_binary(case.i[0].to_string(), case.i[1].to_string());
        if out != case.o.to_string() {
            eprintln!("[ERROR]: Expected {}, but got {}", case.o.to_string(), out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
