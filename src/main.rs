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
        Solution::helper(&nums, 0, nums.len())
    }
    fn helper(nums: &Vec<i32>, start_idx: usize, end_idx: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start_idx == end_idx {
            return None;
        }
        let mid_idx = start_idx + ((end_idx - start_idx) >> 1);
        let mid_num = nums[mid_idx];
        let mut node = TreeNode::new(mid_num);
        node.left = Solution::helper(nums, start_idx, mid_idx);
        node.right = Solution::helper(nums, mid_idx + 1, end_idx);
        Some(Rc::new(RefCell::new(node)))
    }
}
