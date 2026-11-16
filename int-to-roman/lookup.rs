pub struct Solution;
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let symbols = [
            "M","CM","D","CD","C","XC","L","XL","X","IX","V","IV","I"
        ];
        let values = [
            1000,900,500,400,100,90,50,40,10,9,5,4,1
        ];

        let mut out = String::new();
        while num > 0 {
            // Find the biggest possible index
            let mut index = 0;
            // Keep subtracting the greatest from the table that can be
            while index < values.len() {
                if num - values[index] >= 0 {
                    break;
                }
                index += 1;
            }

            num -= values[index];
            out.extend(symbols[index].chars());
        }
        return out;
    }
}
