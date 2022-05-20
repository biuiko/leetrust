use crate::Solution;
use std::collections::HashSet;
impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut sets = HashSet::new();
        for num in nums {
            if sets.contains(&num) {
                return num;
            }
            sets.insert(num);
        }
        -1
    }
}
