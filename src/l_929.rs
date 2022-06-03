use crate::Solution;
use std::collections::HashSet;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails
            .into_iter()
            .map(Self::handle_string)
            .collect::<HashSet<_>>()
            .len() as i32
    }

    fn handle_string(s: String) -> String {
        let s = s.split('@').collect::<Vec<_>>();
        let mut pre = String::new();
        for c in s[0].chars() {
            if c == '+' {
                break;
            }
            if c == '.' {
                continue;
            }
            pre.push(c);
        }
        pre + "@" + s[1]
    }
}
