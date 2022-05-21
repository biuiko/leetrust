use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut ans = (r as i32 - l as i32) * height[l].min(height[r]);
        while l < r - 1 {
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
            ans = ans.max((r as i32 - l as i32) * height[l].min(height[r]));
        }
        ans
    }
}
