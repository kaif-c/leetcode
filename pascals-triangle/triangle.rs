/**
* Okay so pascals triangle
* 1    	     1 + 0
* 1 1  	     1 + 0, 1 + 0
* 1 2 1	     1 + 0, 1 + 1, 1 + 0
* 1 3 3 1    1 + 0, 1 + 1 + 1, 1 + 1 + 1, 1 + 0
* 1 4 6 4 1  1 + 0, 1 + 1 + 1 + 1, 1 + 1 + 1 + 1 + 1 + 1
*
* a = 1
* b = 0
* c = a + b
*
*/
pub struct Solution {}
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        // Exception for first row
        if num_rows == 1 {
            return vec![vec![1]];
        }

        // Annoying cast due to rust
        let row_num = num_rows as usize;
        // The final result vec
        let mut total_val = Vec::with_capacity(row_num);
        // Previous values
        total_val.append(&mut Self::generate(num_rows - 1));

        // Current row
        let mut cur_vec = Vec::with_capacity(row_num);
        for i in 0..(row_num) {
            // Exceptions at edges
            if i == 0 || i == (row_num) - 1 {
                cur_vec.push(1);
                continue;
            }
            
            // Actual computation
            let &last_vec = &total_val.last().unwrap();
            cur_vec.push(last_vec[i - 1] + last_vec[i]);
        }
        total_val.push(cur_vec);
        return total_val;
    }
}
