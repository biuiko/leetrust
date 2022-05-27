use crate::Solution;
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut begin_index = 0;
        let mut l_cnt = 0;
        let mut ans = String::new();
        for (idx, c) in s.chars().enumerate() {
            if c == '(' {
                l_cnt += 1;
            } else {
                l_cnt -= 1;
            }

            if l_cnt == 0 {
                if idx - begin_index > 1 {
                    ans += s.get((begin_index + 1)..=(idx - 1)).unwrap();
                }
                begin_index = idx + 1;
            }
        }
        ans
    }
}
