pub struct Solution;

use std::vec::Vec;
use std::collections::HashMap;

// This approach follows leetcode hints about using two_sum 
// Which upon further inspection is a TERRIBLE idea...
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
       // An approach that finds x + y + z = 0 in REASONABLE time
       // y + z = -x, two_sum can be used
       // but doing it produces copies, for z we get x + y + z, for x we get x + y + z, etc
       // When looking for z, if x OR y have already been looked for we are doing a copy
       // also assuming i = nums[I] for x,y,z then always look for I..nums.len()
       let mut ret: Vec<Vec<i32>> = Vec::new();
       for (index, val_ref) in nums.iter().enumerate() {
           let two_sums = Self::two_sum_from_index_of_neg(&nums, index);
           for two_sum in &two_sums {
               let mut cur_vec = Vec::with_capacity(3);
               if two_sum[0] > *val_ref {
                   cur_vec.extend([*val_ref, two_sum[0], two_sum[1]]);
               }
               else if two_sum[1] < *val_ref {
                   cur_vec.extend([two_sum[0], two_sum[1], *val_ref]);
               }
               else {
                   cur_vec.extend([two_sum[0], *val_ref, two_sum[1]]);
               }

               let mut is_unique = true;
               for cur_ret in &ret {
                   if cur_ret[0] == cur_vec[0] && cur_ret[1] == cur_vec[1] {
                       is_unique = false;
                       break;
                   }
               }
               if is_unique {
                   ret.push(cur_vec);
               }
           }
       }
       return ret;
    }

    fn two_sum_from_index_of_neg(nums: &Vec<i32>, sum_index: usize) -> Vec<[i32; 2]> {
        // Find the 2 sum
        // As they have to be unique simply loop over the rest
        let mut map = HashMap::<i32, i32>::new();
        let mut ret = Vec::new();
        // We do > sum_index + 1 as < sum_index has already been looked for by them
        for index in (sum_index + 1)..nums.len() {
            let mapped = map.get(&nums[index]);
            if let Some(other) = mapped {
                if nums[index] < *other {
                    ret.push([nums[index], *other]);
                }
                else {
                    ret.push([*other, nums[index]]);
                }
            }
            map.insert(-nums[sum_index] - nums[index], nums[index]);
        }
        return ret;
    }
}
