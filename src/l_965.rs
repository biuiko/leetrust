// 复习 BFS 了

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use crate::Solution;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = match root {
            Some(r) => r,
            None => {
                return true;
            }
        };
        let v = (*root).borrow().val;
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let left = &(*node).borrow().left;
            if let Some(left) = left {
                if (*left).borrow().val != v {
                    return false;
                }
                queue.push_back(left.clone());
            }
            let right = &(*node).borrow().right;
            if let Some(right) = right {
                if (*right).borrow().val != v {
                    return false;
                }
                queue.push_back(right.clone());
            }
        }
        true
    }
}
