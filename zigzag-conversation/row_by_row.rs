pub struct Solution;
impl Solution {
    // Space = O(n)
    // Time = O(n)
    pub fn convert(s: String, num_rows: i32) -> String {
        // INFO:
        // |  /|
        // | / |
        // |   |
        // where veritcal is the rows
        // the string returned must be row by row
        // Vertical = num_rows & inclined line len = vertical line len
        // Filler words must be num_rows - 2
        // Next horizontal = num_rows + (num_rows - 1) <- for top

        if num_rows == 1 {
            return s;
        }

        let mut out = String::with_capacity(s.len());
        for cur_row in 0..num_rows {
            let mut index = cur_row;
            let inc = num_rows * 2 - 2 - 2 * cur_row;
            while index < s.len() as i32 {
                // Push the columns
                out.push(s.as_bytes()[index as usize] as char);
                let inc_ind = index + inc;
                // Calculate next columns same rows index
                index += num_rows * 2 - 2;
                // Push the inclined lines
                if cur_row > 0 && cur_row < num_rows - 1 && inc_ind < s.len() as i32 {
                    out.push(s.as_bytes()[inc_ind as usize] as char);
                }
            }
        }
        return out;
    }
}
