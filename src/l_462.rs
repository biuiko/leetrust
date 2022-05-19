use crate::Solution;
impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let len = nums.len();
        let target = if len % 2 == 1 {
            nums[len / 2]
        } else {
            (nums[len / 2] + nums[len / 2 - 1]) / 2
        };
        nums.iter().fold(0, |sum, v| sum + (target - v).abs())
    }
}
