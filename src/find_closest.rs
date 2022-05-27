use crate::Solution;
impl Solution {
    pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
        const MAXN: usize = 100005;
        let mut idx1 = MAXN;
        let mut idx2 = MAXN;
        let mut ans = words.len() as i32;
        for (idx, word) in words.into_iter().enumerate() {
            if word1 == word {
                idx1 = idx;
                if idx2 != MAXN {
                    ans = ans.min((idx1 - idx2) as i32);
                }
            } else if word2 == word {
                idx2 = idx;
                if idx1 != MAXN {
                    ans = ans.min((idx2 - idx1) as i32);
                }
            }
        }
        ans
    }
}
