mod triangle;

struct Case {
    pub rows: i32,
    pub expected: Vec<Vec<i32>>,
}
pub fn main() {
    let cases = [
        Case { rows: 1, expected: vec![vec![1]] },
        Case { rows: 5, expected:
            vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]] },
    ];
    for case in cases {
        let val = crate::triangle::Solution::generate(case.rows);
        if val != case.expected {
            eprintln!("FAILED: Expected {:?}; Got {:?}", case.expected, val);
            continue;
        }
        println!("SUCCESS");
    }
}
