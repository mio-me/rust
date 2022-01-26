use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution;
impl Solution {
  pub fn get_all_elements(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
  ) -> Vec<i32> {
    let mut res = Vec::new();
    let mut stack = vec![root1, root2];
    while let Some(node) = stack.pop() {
      if let Some(node) = node {
        let node = node.borrow();
        res.push(node.val);
        stack.push(node.left.clone());
        stack.push(node.right.clone());
      }
    }
    res.sort_unstable();
    res
  }
}
