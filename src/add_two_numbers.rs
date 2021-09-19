// Definition for singly-linked list.
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

/*
Runtime: 23 ms, faster than 5.05% of Rust online submissions for Add Two Numbers.
Memory Usage: 2 MB, less than 90.24% of Rust online submissions for Add Two Numbers.
*/
pub fn add_two_numbers(
 l1: Option<Box<ListNode>>,
 l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {

  return do_sum(l1, l2, None);
}

pub fn do_sum(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, in_memory: Option<i32>) -> Option<Box<ListNode>> {
  match (l1, l2, in_memory) {
    (None, None, None) => None,
    (None, None, Some(_)) => Some(Box::new(ListNode::new(1))),
    (Some(n), None, None) => {
      let mut b = Box::new(ListNode::new(n.val));
      b.next = do_sum(n.next, None, None);
      Some(b)      
    },
    (Some(n), None, Some(_)) => {
      if n.val + 1 < 10 {
        let mut b = Box::new(ListNode::new(n.val + 1));
        b.next = do_sum(n.next, None, None);
        Some(b)
      } else {
        let mut b = Box::new(ListNode::new(n.val +1 - 10));
        b.next = do_sum(n.next, None, Some(1));
        Some(b)
      } 
    },
    (None, Some(n), None) => {
      let mut b = Box::new(ListNode::new(n.val));
      b.next = do_sum(None, n.next, None);
      Some(b)      
    },
    (None, Some(n), Some(_)) => {
      if n.val + 1 < 10 {
        let mut b = Box::new(ListNode::new(n.val + 1));
        b.next = do_sum(None,n.next, None);
        Some(b)
      } else {
        let mut b = Box::new(ListNode::new(n.val + 1 - 10));
        b.next = do_sum(None, n.next, Some(1));
        Some(b)
      } 
    },
    (Some(n1), Some(n2), None) => {
      if n1.val + n2.val < 10 {
        let mut b = Box::new(ListNode::new(n1.val + n2.val));
        b.next = do_sum(n1.next,n2.next, None);
        Some(b)
      } else {
        let mut b = Box::new(ListNode::new(n1.val + n2.val - 10));
        b.next = do_sum(n1.next, n2.next, Some(1));
        Some(b)
      } 
    },
    (Some(n1), Some(n2), Some(_)) => {
      if n1.val + n2.val + 1 < 10 {
        let mut b = Box::new(ListNode::new(n1.val + n2.val + 1));
        b.next = do_sum(n1.next,n2.next, None);
        Some(b)
      } else {
        let mut b = Box::new(ListNode::new(n1.val + n2.val + 1 - 10));
        b.next = do_sum(n1.next, n2.next, Some(1));
        Some(b)
      } 
    },
  }
}