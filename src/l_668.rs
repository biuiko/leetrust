use crate::Solution;
impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut l = 1;
        let mut r = m * n;
        while l < r {
            let mid = (l + r) / 2;
            let mut cnt = (mid / n) * n;
            for i in (mid / n + 1)..=m {
                cnt += mid / i;
            }
            println!("mid {} cnt {}", mid, cnt);
            if cnt < k {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        r
    }
}
