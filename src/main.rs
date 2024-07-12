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
    let root = Some(Rc::new(RefCell::new(
        TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(
                TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(9))))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(10)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(11))))
                    }))),
                }
            ))),
            right: Some(Rc::new(RefCell::new(
                TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(12)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(13))))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(14)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(15))))
                    }))),
                }
            )))
        }
    )));
    assert_eq!(Solution::count_nodes(root), 15);
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
        if root.is_none() {
            return 0;
        }
        let mut level = -1;
        let mut node = root.clone();
        while let Some(value) = node {
            level += 1;
            node = value.borrow().left.clone();
        }
        let mut low = 1 << level;
        let mut high = (1 << (level + 1)) - 1;
        while low < high {
            let mid = (high - low + 1) / 2 + low;
            if Self::exists(root.clone(), level, mid) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        low
    }
    fn exists(root: Option<Rc<RefCell<TreeNode>>>, level: i32, k: i32) -> bool {
        let mut bits = 1 << (level - 1);
        let mut node = root;
        while node.is_some() && bits > 0 {
            node = if bits & k > 0 {
                node.unwrap().borrow().right.clone()
            } else {
                node.unwrap().borrow().left.clone()
            };
            bits >>= 1;
        }
        node.is_some()
    }
}