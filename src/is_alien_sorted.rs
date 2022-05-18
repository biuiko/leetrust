use crate::Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut orders = vec![0; 26];
        for (order, v) in order.chars().enumerate() {
            orders[v as usize - 'a' as usize] = order;
        }
        for i in 1..words.len() {
            let mut r = words[i].chars();
            let mut l = words[i - 1].chars();
            loop {
                let lc = l.next();
                let rc = r.next();
                if lc.is_none() {
                    break;
                }
                if rc.is_none() {
                    return false;
                }
                if orders[lc.unwrap() as usize - 'a' as usize]
                    > orders[rc.unwrap() as usize - 'a' as usize]
                {
                    return false;
                } else if orders[lc.unwrap() as usize - 'a' as usize]
                    < orders[rc.unwrap() as usize - 'a' as usize]
                {
                    break;
                }
            }
        }
        return true;
    }
}
