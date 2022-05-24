use crate::Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let s_len = s.len() as i32;
        if s_len <= num_rows || num_rows == 1 {
            return s;
        }
        let chars = s.chars().collect::<Vec<_>>();
        let mut ans = String::default();
        for i in 0..num_rows {
            for j in 0..s_len {
                let idx = if i == 0 {
                    j * 2 * (num_rows - 1)
                } else if i == num_rows - 1 {
                    j * 2 * (num_rows - 1) + (num_rows - 1)
                } else if j % 2 == 0 {
                    i + (j + 1) / 2 * (num_rows - 1) * 2
                } else {
                    (j + 1) / 2 * (num_rows - 1) * 2 - i
                };
                if idx < 0 {
                    continue;
                }
                if idx >= s_len {
                    break;
                }
                ans.push(chars[idx as usize]);
            }
        }
        ans
    }
}
