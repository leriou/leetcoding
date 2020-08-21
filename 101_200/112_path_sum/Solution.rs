// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match root {
            Some(node) => {
                match (
                    node.borrow().left.as_ref().map(|x| x.clone()),
                    node.borrow().right.as_ref().map(|x| x.clone()),
                ) {
                    (None, None) => sum == node.borrow().val,
                    (left, right) => {
                        Self::has_path_sum(left, sum - node.borrow().val)
                            || Self::has_path_sum(right, sum - node.borrow().val)
                    }
                }
            }
            None => false,
        }
    }
}
