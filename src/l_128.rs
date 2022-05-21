use crate::Solution;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let sets: HashSet<i32> = HashSet::from_iter(nums.clone());

        let mut mp = nums.iter().map(|num| (*num, 0)).collect::<HashMap<_, _>>();
        let mut ans = 0;
        for k in sets.into_iter() {
            let v = *mp.get(&k).unwrap();
            let len = if v == 0 { find_x(k, &mut mp) } else { v };
            ans = ans.max(len);
        }
        ans
    }
}

fn find_x(num: i32, mp: &mut HashMap<i32, i32>) -> i32 {
    let v = mp.get(&num).map(|x| *x);
    match v {
        None => {
            return 0;
        }
        Some(x) => {
            if x != 0 {
                return x;
            } else {
                let len = find_x(num - 1, mp);
                mp.insert(num, 1 + x + len);
                // println!("num {:?}, mp {:?}",num, mp);
                return x + 1 + len;
            }
        }
    }
}
