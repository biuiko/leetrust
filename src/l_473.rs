use crate::Solution;
impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if sum % 4 != 0 {
            return false;
        }
        let target = sum / 4;
        let mut lines = vec![0; 4];
        // 优化一下 搜索的时候快点返回
        matchsticks.sort_by(|a, b| b.cmp(a));
        return dfs(0, &mut lines, target, &matchsticks);
    }
}

fn dfs(idx: usize, lines: &mut Vec<i32>, target: i32, matchsticks: &[i32]) -> bool {
    if idx == matchsticks.len() {
        return true;
    }
    for i in 0..4 {
        lines[i] += matchsticks[idx];
        if lines[i] <= target && dfs(idx + 1, lines, target, matchsticks) {
            return true;
        }
        lines[i] -= matchsticks[idx];
    }
    return false;
}
