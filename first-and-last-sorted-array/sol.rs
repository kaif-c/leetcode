pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // The array is sorted, so loop over till target found
        // Must be O(log N) so binary lookup

        let mut ret = Vec::<i32>::with_capacity(2);
        // Early termination
        if nums.len() == 0 {
            ret.extend(&[-1, -1]);
            return ret;
        }

        let mut min_ind = 0;
        let mut max_ind = nums.len();
        let mut mid;
        // Binary search to find the minimum range
        // as well as if the target even exists or not
        loop {
            if min_ind >= max_ind {
                ret.extend(&[-1, -1]);
                return ret;
            }

            mid = min_ind / 2 + max_ind / 2;
            if nums[mid] < target {
                min_ind = mid + 1;
                continue;
            }
            if nums[mid] > target {
                max_ind = mid;
                continue;
            }

            // We found the minimum
            if mid == 0 {
                min_ind = 0;
                ret.push(mid as i32);
                break;
            }
            if nums[mid - 1] != target {
                min_ind = mid;
                ret.push(mid as i32);
                break;
            } 
            // We found the target but its not its first occurance
            max_ind = mid;
        }

        // Reset the max
        max_ind = nums.len();
        // Find the max range
        while min_ind < max_ind {
            mid = min_ind / 2 + max_ind / 2;
            if nums[mid] < target {
                min_ind = mid + 1;
                continue;
            }
            if nums[mid] > target {
                max_ind = mid;
                continue;
            }
            // We found the maximum
            if mid == nums.len() - 1 {
                ret.push(mid as i32);
                break;
            }
            if nums[mid + 1] != target {
                ret.push(mid as i32);
                break;
            } 
            // We found the target but its not its last occurance
            min_ind = mid + 1;
        }
        return ret;
    }
}
