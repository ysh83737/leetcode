fn main() {
    let root = Some(Rc::new(RefCell::new(
        TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(
                TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                }
            ))),
            right: Some(Rc::new(RefCell::new(
                TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: None
                }
            )))
        }
    )));
    assert_eq!(Solution::count_nodes(root), 6);
}

struct Solution;

// Definition for a binary tree node.
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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        if let Some(value) = root {
            result += 1;
            let node = value.borrow();
            let left = node.left.clone();
            let right = node.right.clone();
            if left != None {
                result += Solution::count_nodes(left);
            }
            if right != None {
                result += Solution::count_nodes(right);
            }
        }
        result
    }
}