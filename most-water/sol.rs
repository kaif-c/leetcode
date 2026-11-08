pub struct Solution;
use std::vec::Vec;
use std::cmp::min;

impl Solution {
    // Time = O(n)
    // Space = O(1)
    pub fn max_area(height: Vec<i32>) -> i32 {
        // INFO:
        // 2 pointers
        // min & max
        // move the one with the lower height towards the greater one
        // move max towards min otherwise
        // Keep the largest possible area (Like Kadane's Algorithm)
        let mut max_ind = height.len() - 1;
        let mut min_ind = 0usize;
        let mut area = 0i32;

        while min_ind < max_ind {
            // Calculate area
            let h = min(height[min_ind], height[max_ind]);
            let w = (max_ind - min_ind) as i32;
            let cur_area = h * w;
            if cur_area > area {
                area = cur_area;
            }

            // Move whichever has the lowest height
            if height[min_ind] < height[max_ind] {
                min_ind += 1;
                continue;
            }
            max_ind -= 1;
        }
        return area;
    }
}
