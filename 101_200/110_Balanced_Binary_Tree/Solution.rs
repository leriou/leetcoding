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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let p = node.as_ptr();
                unsafe {
                    let left_ref = (*p).left.as_ref();
                    let right_ref = (*p).right.as_ref();
                    Self::is_balanced(left_ref.map(|x| x.clone()))
                        && Self::is_balanced(right_ref.map(|x| x.clone()))
                        && (Self::helper(left_ref.map(|x| x.clone()))
                            - Self::helper(right_ref.map(|x| x.clone())))
                        .abs()
                            <= 1
                }
            }
            None => true,
        }
    }
    fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let p = node.as_ptr();
                unsafe {
                    let left = (*p).left.as_ref().map(|x| x.clone());
                    let right = (*p).right.as_ref().map(|x| x.clone());
                    Self::helper(left).max(Self::helper(right)) + 1
                }
            }
            None => 0,
        }
    }
}
