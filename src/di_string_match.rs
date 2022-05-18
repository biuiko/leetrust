use crate::Solution;
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut l = 0;
        let mut r = s.len() as i32;
        let mut ans = Vec::with_capacity(s.len() + 1);
        for c in s.chars() {
            if c == 'D' {
                ans.push(r);
                r -= 1;
            } else {
                ans.push(l);
                l += 1;
            }
        }
        ans.push(l);
        ans
    }
}
