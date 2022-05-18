#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::helper(l1, l2, 0)
    }

    fn helper(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        mut carry: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            None
        } else {
            let mut val = carry;
            l1.as_ref().map(|l| val += l.val);
            l2.as_ref().map(|l| val += l.val);
            if val >= 10 {
                val -= 10;
                carry = 1;
            } else {
                carry = 0;
            }
            Some(Box::new(ListNode {
                val,
                next: Self::helper(l1.and_then(|l| l.next), l2.and_then(|l| l.next), carry),
            }))
        }
    }
}
