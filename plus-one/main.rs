mod right_left;

struct Case {
    pub val: Vec<i32>,
    pub expected_res: Vec<i32>
}

pub fn main() {
    let cases = [
        Case {val: vec![1, 2, 3], expected_res: vec![1, 2, 4]},
        Case {val: vec![1, 9, 9], expected_res: vec![2, 0, 0]},
        Case {val: vec![9],       expected_res: vec![1, 0]},
    ];
    
    for case in cases {
        let res = crate::right_left::Solution::plus_one(case.val);
        if res == case.expected_res {
            println!("Success");
            continue;
        }

        println!("FAILURE: Expected: {:?}, but got {:?}", case.expected_res, res);
    }
}
