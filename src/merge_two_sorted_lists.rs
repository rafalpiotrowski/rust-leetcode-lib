// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

/*
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Merge Two Sorted Lists.
Memory Usage: 2.1 MB, less than 22.05% of Rust online submissions for Merge Two Sorted Lists.
*/
pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    merge_two_lists_as_refs(&l1, &l2)
}

pub fn merge_two_lists_as_refs(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => {
            let mut node = Box::new(ListNode::new(n.val));
            node.next = merge_two_lists_as_refs(&n.next, &None);
            Some(node)
        },
        (Some(n1), Some(n2)) => {
            if n1.val > n2.val {
                let mut node = Box::new(ListNode::new(n2.val));
                node.next = merge_two_lists_as_refs(l1, &n2.next);
                Some(node)
            } else {
                let mut node = Box::new(ListNode::new(n1.val));
                node.next = merge_two_lists_as_refs(&n1.next, l2);
                Some(node)
            } 
        },
    }
}
