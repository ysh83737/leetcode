fn main() {
    let mut nums = vec![-10,-3,0,5,9];
    let result = Solution::sorted_array_to_bst(nums);
    println!("{:?}", result);
}

struct Solution {}

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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }
        let mid_idx = nums.len() >> 1;
        let mid_num = nums[mid_idx];
        let mut node = TreeNode::new(mid_num);
        node.left = Solution::sorted_array_to_bst(nums[0..mid_idx].to_vec());
        node.right = Solution::sorted_array_to_bst(nums[(mid_idx + 1)..].to_vec());
        Some(Rc::new(RefCell::new(node)))
    }
}
