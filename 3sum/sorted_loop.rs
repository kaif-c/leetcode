pub struct Solution;
use std::collections::HashMap;

impl Solution {
    // Following implementation works but is just REALLY slow
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = vec![];
        for (index, val) in nums.iter().enumerate() {
            let mut cur_sum = Self::two_sum(&nums, index);
            for s in &mut cur_sum {
                if *val > s[1] {
                    s.push(*val);
                }
                else if *val < s[0] {
                    s.insert(0, *val)
                }
                else {
                    s.insert(1, *val);
                }
            }
            if cur_sum.len() > 0 {
                ret.extend(cur_sum);
            }
        }
        let mut trimmed_ret: Vec<Vec<i32>> = Vec::with_capacity(ret.len());
        for r in &ret {
            let mut can_mut = true;
            for i in &trimmed_ret {
                if *i == *r {
                    can_mut = false;
                }
            }

            if can_mut {
                trimmed_ret.push(r.clone());
            }
        }
        return trimmed_ret;
    }

    fn two_sum(nums: &Vec<i32>, sum_index: usize) -> Vec<Vec<i32>> {
        let mut ret = Vec::with_capacity(3);
        let mut map = HashMap::<i32, i32>::new();
        for index in (sum_index+1)..nums.len() {
            if map.contains_key(&nums[index]) {
                ret.push(vec![*map.get(&nums[index]).unwrap(), nums[index]]);
            }
            map.insert(-nums[sum_index] - nums[index], nums[index]);
        }
        for cur in &mut ret {
            if cur[0] > cur[1] {
                cur[0] ^= cur[1];
                cur[1] ^= cur[0];
                cur[0] ^= cur[1];
            }
        }
        return ret;
    }
}
