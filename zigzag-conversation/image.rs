pub struct Solution;

use std::vec::Vec;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // INFO:
        // |  /|
        // | / |
        // |   |
        // where veritcal is the rows
        // the string returned must be row by row
        // Vertical = num_rows & inclined line len = vertical line len
        // Create the 2D image and simply join row by row

        if num_rows == 1 {
            return s;
        }
        let mut img = Vec::with_capacity(num_rows as usize);
        for _ in 0..num_rows {
            img.push("".to_string());
        }
        let mut cur_row = 0i32;
        let mut direction = 1i32;
        for read_ind in 0..s.len() {
            img[cur_row as usize].push(
                s.as_bytes()[read_ind] as char
            );
            cur_row += direction;
            if cur_row < 0 || cur_row == num_rows {
                direction *= -1;
                cur_row += direction * 2;
            }
        }

        return img.concat();
    }
}
