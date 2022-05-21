// 没学下去马拉车
// 话说马拉车的英文是 manacher
// 一些 n l 不分

// ps: 类型转换好烦哦

use crate::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let chars = s.chars().collect::<Vec<_>>();
        let mut start = 0;
        let mut end = 0;
        for i in 0..(len - 1) {
            let (s1, e1) = Self::find_max_len(&chars, i as i32, i);
            if e1 - s1 > end - start {
                end = e1;
                start = s1;
            }
            let (s1, e1) = Self::find_max_len(&chars, i as i32, i + 1);
            if e1 - s1 > end - start {
                end = e1;
                start = s1;
            }
        }
        s.get(start..=end).unwrap().to_string()
    }

    fn find_max_len(chars: &[char], mut l: i32, mut r: usize) -> (usize, usize) {
        let len = chars.len();
        let mut end = l as usize;
        let mut start = l;
        while l >= 0 && r < len {
            if chars[l as usize] == chars[r] {
                end = r;
                start = l;
                l -= 1;
                r += 1;
            } else {
                break;
            }
        }
        (start as usize, end)
    }
}
