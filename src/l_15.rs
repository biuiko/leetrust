use crate::Solution;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans = Vec::new();
        let len = nums.len();
        for first in 0..len {
            if first > 0 && nums[first] == nums[first - 1] {
                continue;
            }
            let mut third = len - 1;
            for second in first + 1..len - 1 {
                if second > first + 1 && nums[second] == nums[second - 1] {
                    continue;
                }
                while third > second && nums[third] + nums[second] + nums[first] >= 0 {
                    if nums[third] + nums[second] + nums[first] == 0 {
                        ans.push(vec![nums[first], nums[second], nums[third]]);
                        while third > second + 1 && nums[third] == nums[third - 1] {
                            third -= 1;
                        }
                        break;
                    } else {
                        third -= 1;
                    }
                }
            }
        }
        ans
    }
}
