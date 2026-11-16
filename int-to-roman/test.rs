mod lookup;
use crate::lookup::Solution;
use std::process::ExitCode;

struct Case {
    pub i: i32,
    pub o: String,
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i: 5, o: "V".to_string()},
        Case {i: 9, o: "IX".to_string()},
        Case {i: 3749, o: "MMMDCCXLIX".to_string()},
        Case {i: 49, o: "XLIX".to_string()},
        Case {i: 58,   o: "LVIII".to_string()},
        Case {i: 994, o: "CMXCIV".to_string()},
        Case {i: 1994, o: "MCMXCIV".to_string()},
    ];

    let mut errors = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::int_to_roman(case.i);
        if out != case.o {
            eprintln!("[ERROR]: Expected {:?}, but got {:?}", case.o, out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
