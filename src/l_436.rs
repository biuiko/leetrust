use crate::Solution;
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let len = intervals.len();
        let mut left = Vec::with_capacity(len);
        let mut right = Vec::with_capacity(len);
        let mut ans = vec![-1; len];

        for (idx, value) in intervals.into_iter().enumerate() {
            left.push(Node { idx, v: value[0] });
            right.push(Node { idx, v: value[1] });
        }
        left.sort_by(|a, b| a.v.cmp(&b.v));
        right.sort_by(|a, b| a.v.cmp(&b.v));
        let mut l = 0;
        for r in right {
            while l < len {
                if left[l].v < r.v {
                    l += 1;
                } else {
                    ans[r.idx] = left[l].idx as i32;
                    break;
                }
            }
        }
        ans
    }
}

#[derive(Debug)]
pub struct Node {
    idx: usize,
    v: i32,
}
