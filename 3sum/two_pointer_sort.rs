pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 2 {
            return Vec::new();
        }
        // A sorting followed by 2 pointer approach
        nums.sort_unstable();
        let mut ret: Vec<Vec<i32>> = Vec::new();

        for index in 0..(nums.len() - 2) {
            // Simple 2sum
            // Vec already sorted so we move closer in the direction where we get closer to it
            if index > 0 && nums[index] == nums[index - 1] {
                continue;
            }
            let mut min_ind = index + 1;
            let mut max_ind = nums.len() - 1;
            while min_ind < max_ind {
                let sum = nums[index] + nums[min_ind] + nums[max_ind];
                // No need for overflow checks as min_ind < max_ind
                if sum < 0 {
                    min_ind += 1;
                    continue;
                }
                if sum > 0 {
                    max_ind -= 1;
                    continue;
                }

                ret.push(vec![nums[index], nums[min_ind], nums[max_ind]]);
                max_ind -= 1;
                min_ind += 1;
                while min_ind < max_ind && nums[min_ind] == nums[min_ind - 1] {
                    min_ind += 1;
                }
                while min_ind < max_ind && nums[max_ind] == nums[max_ind + 1] {
                    max_ind -= 1;
                }
            }
        }
        return ret;
    }
}
