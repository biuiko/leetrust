use crate::Solution;
use std::cmp::max;
impl Solution {
    pub fn jump(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut max_position = 0;
        let mut end = 0;
        for (index, num) in nums.iter().enumerate() {
            if index == nums.len() - 1 {
                break;
            }
            max_position = max(max_position, index + (*num as usize));
            if index == end {
                ans += 1;
                end = max_position;
            }
        }
        ans
    }
}
