mod looped_longest;
use crate::looped_longest::Solution;
use std::process::ExitCode;

struct Case {
    pub i: &'static str,
    pub o: i32,
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i: "Hello World", o: 5},
        Case {i: "   fly me   to   the moon  ", o: 4},
        Case {i: "luffy is still joyboy", o: 6},
    ];

    let mut errors: ExitCode = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::length_of_last_word(case.i.to_string());
        if out != case.o {
            eprintln!("Expected {} but got {out}", case.o);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
