use crate::Solution;
impl Solution {
    pub fn one_edit_away(first: String, second: String) -> bool {
        let (first, second) = if first.len() > second.len() {
            (first, second)
        } else {
            (second, first)
        };
        let first = first.chars().collect::<Vec<_>>();
        let second = second.chars().collect::<Vec<_>>();
        if first.len() - second.len() > 1 {
            return false;
        };
        let len = second.len();
        if first.len() == second.len() {
            let mut d = 0;
            for i in 0..len {
                if first[i] != second[i] {
                    d += 1;
                };
                if d > 1 {
                    return false;
                }
            }
        } else {
            let mut d = 0;
            let mut j = 0;
            let mut offset = 0;
            for i in 0..len {
                if first[i + offset] != second[i] {
                    d += 1;
                    offset = 1;
                    if first[offset + i] != second[i] {
                        d += 1;
                    }
                }
                if d > 1 {
                    return false;
                }
                j = j + 1;
            }
        }
        return true;
    }
}
