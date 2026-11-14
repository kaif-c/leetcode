pub struct Solution;

use std::cmp::min;
impl Solution {
    // Time = O(n*r^2) where n = nums.len()
    // Space = O(1)
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        // INFO:
        // For each element in the array
        // as long as index < size - l
        // create a subarray slice from index -> l
        // calculate the sum of this slice
        // increment its size till it becomes index -> r
        // if sum found then check if its smaller than last found sum

        let mut sum = -1i32;
        // As l is also lenght we use inclusive range
        for index in 0..=(nums.len() - l as usize + 1) {
            // Loop that calculates each max index index + l -> min(r, nums.len())
            for cur_max_ind in (index + l as usize - 1)..min(index + r as usize, nums.len()) {
                // Sum
                let cur_sum: i32 = (&nums[index..=cur_max_ind]).iter().sum();
                // Check
                if cur_sum <= 0 {
                    continue;
                }
                if cur_sum < sum || sum == -1 {
                    sum = cur_sum;
                }
            }
        }
        return sum;
    }
}
