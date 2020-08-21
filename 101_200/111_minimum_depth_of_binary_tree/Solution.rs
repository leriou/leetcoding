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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                match (
                    node.borrow().left.as_ref().map(|x| x.clone()),
                    node.borrow().right.as_ref().map(|x| x.clone()),
                ) {
                    (Some(left), None) => Self::min_depth(Some(left)) + 1,
                    (None, Some(right)) => Self::min_depth(Some(right)) + 1,
                    (Some(left), Some(right)) => {
                        Self::min_depth(Some(left)).min(Self::min_depth(Some(right))) + 1
                    }
                    (None, None) => 1,
                }
            }
            None => 0,
        }
    }
}
