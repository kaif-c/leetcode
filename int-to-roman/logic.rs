pub struct Solution;
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        // num < 399 and > 1
        let char_table = [
            'I',
            'V',
            'X',
            'L',
            'C',
            'D',
            'M'
        ];
        let val_table = [
            1,
            5,
            10,
            50,
            100,
            500,
            1000
        ];

        let mut out = String::new();
        while num > 0 {
            // Find the biggest possible index
            let mut index = val_table.len();
            // Keep subtracting the greatest from the table that can be
            while index > 1 {
                if num - val_table[index - 1] >= 0 {
                    break;
                }
                index -= 1;
            }

            // This checks for subtractive form for 4 (IIII -> IV)
            // As a side-effect of the algorythm,
            // its only possible for the picked value to be 10^n
            if index != val_table.len() && num >= 4 * val_table[index - 1] {
                num -= 4 * val_table[index - 1];
                out.extend([
                    char_table[index - 1],
                    char_table[index]
                ]);
                continue;
            }

            // If the index is even then val is 5*10^n
            // This checks for subtractive form of 9
            // As the biggest value is 5
            if index != val_table.len() && index % 2 == 0 && num >= val_table[index] - val_table[index - 2] {
                num -= val_table[index] - val_table[index - 2];
                out.extend([
                    char_table[index - 2],
                    char_table[index]
                ]);
                continue;
            }

            num -= val_table[index - 1];
            out.push(char_table[index - 1]);
        }
        return out;
    }
}
