use std::collections::HashMap;
pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.chars().into_iter().collect::<Vec<_>>();
    let mut ans: i32 = 0;
    let mut mp = HashMap::new();
    let mut start = 0;

    for end in 0..s.len() {
        if mp.contains_key(&s[end]) {
            start = start.max(*mp.get(&s[end]).unwrap() + 1);
        }
        mp.insert(s[end], end as i32);
        ans = ans.max(end as i32 - start + 1);
    }
    ans
}
