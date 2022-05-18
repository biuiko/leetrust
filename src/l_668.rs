use crate::Solution;
impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut l = 1;
        let mut r = m * n;
        let mut ans = (l + r) / 2;
        while l < r {
            ans = (l + r) / 2;
            let mut cnt = (ans / n) * n;
            for i in (ans / n + 1)..=m {
                cnt += ans / i;
            }
            println!("mid {} cnt {}", ans,cnt);
            if cnt < k {
                l = ans + 1;
            } else {
                r = ans;
            }
        }
        r
    }
}
