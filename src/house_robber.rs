use crate::Solution;
use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut not_take = 0;
        let mut take = 0;
        for num in nums {
            let pre_take = take;
            take = max(not_take + num, take);
            not_take = pre_take;
        }
        max(not_take, take)
    }
}
