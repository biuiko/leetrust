use crate::Solution;

impl Solution {
    // pub fn reverse(x: i32) -> i32 {
    //     let is_positive = x>=0;
    //     let s = (x as i64).abs().to_string().chars().rev().collect::<String>();
    //     let ans = s.parse::<i32>().unwrap_or_default();
    //     if is_positive{
    //         ans
    //     }else{
    //         0 as i32 - ans
    //     }
    // }

    pub fn reverse(mut x: i32) -> i32 {
        let mut ans = 0;
        while x != 0 {
            if ans > i32::MAX / 10 || ans < i32::MIN / 10 {
                return 0;
            }
            ans = ans * 10 + x % 10;
            x /= 10;
        }
        ans
    }
}
