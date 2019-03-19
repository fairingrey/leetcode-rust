// https://leetcode.com/problems/invert-binary-tree/

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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(v) => {
                let mut v = v.borrow_mut();

                let val = v.val;

                let temp = v.left.clone();
                v.left = v.right.clone();
                v.right = temp;
                let left = Solution::invert_tree(v.left.clone());
                let right = Solution::invert_tree(v.right.clone());

                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            }
            None => None,
        }
    }
}
