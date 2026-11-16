pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let table = [
            "abc",
            "def",
            "ghi",
            "jkl",
            "mno",
            "pqrs",
            "tuv",
            "wxyz",
        ];
        return Self::current_letter_combinations(&digits[..], &table);
    }
    fn current_letter_combinations(digits: &str, table: &[&str]) -> Vec<String> {
        let mut old_vec = Vec::<String>::new();
        if digits.len() > 1 {
            old_vec = Self::current_letter_combinations(&digits[1..], table);
        }

        let index = *digits.as_bytes().first().unwrap() as usize - '2' as usize;

        if old_vec.len() == 0 {
            for c in table[index].chars() {
                old_vec.push(c.to_string());
            }
            return old_vec;
        }
        
        let old_vec_len = old_vec.len();
        let mut new_vec = Vec::<String>::new();
        for i in 0..table[index].len() {
            new_vec.extend(old_vec.clone());
        }
        
        let mut cur_vec_offset = 0usize;
        for cur_char_ind in 0..table[index].len() {
            for i in 0..old_vec_len {
                new_vec[i + cur_vec_offset].insert(0, table[index].as_bytes()[cur_char_ind] as char);
            }
            cur_vec_offset += old_vec_len;
        }
        return new_vec;
    }
}
