mod sol;
use crate::sol::Solution;
use std::process::ExitCode;
use std::vec::Vec;

struct Case {
    pub i: String,
    pub o: Vec<String>,
}

macro_rules! stv {
    ($($x:expr),* $(,)?) => {
        vec![$($x.to_string()),*]
    };
}

/// Checks if 2 vectors have the same elements, the order of elements can be different
fn vec_eq<T: Ord + Clone>(a: &Vec<T>, b: &Vec<T>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut a = a.clone();
    let mut b = b.clone();

    a.sort();
    b.sort();

    a == b
}

pub fn main() -> ExitCode {
    let cases = [
        Case {i: "23".to_string(), o: stv!["ad","ae","af","bd","be","bf","cd","ce","cf"]},
        Case {i: "2".to_string(), o: stv!["a", "b", "c"]},
        Case {i: "7".to_string(), o: stv!["p", "q", "r", "s"]},
    ];

    let mut errors = ExitCode::SUCCESS;
    for case in cases {
        let out = Solution::letter_combinations(case.i);
        if !vec_eq(&out, &case.o) {
            eprintln!("[ERROR]: Expected {:?}, but got {:?}", case.o, out);
            errors = ExitCode::FAILURE;
            continue;
        }
        println!("SUCCESS");
    }
    return errors;
}
