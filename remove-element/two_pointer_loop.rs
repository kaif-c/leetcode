pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // i want Space = O(1)
        // 2 pointer solution?
        // 1 where to write, 1 where to read
        // if same as val read ind increases but not write
        // else writes and reads and both increases
        // return read index

        let mut write_ind = 0usize;
        for read_ind in 0..nums.len() {
            if nums[read_ind] != val {
                nums[write_ind] = nums[read_ind];
                write_ind += 1;
            }
        }

        // We know the range so it works
        return write_ind as i32;
    }
}
