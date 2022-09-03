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
  pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    match [list1, list2] {
      [Some(mut a), Some(mut b)] => {
        if b.val < a.val {
          b.next = Solution::merge_two_lists(Some(a), b.next);
          Some(b)
        } else {
          a.next = Solution::merge_two_lists(Some(b), a.next);
          Some(a)
        }
      }
      [Some(x), None] | [None, Some(x)] => Some(x),
      _ => None,
    }
  }
}
