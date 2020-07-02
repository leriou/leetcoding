// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut curr, mut prev) = (head, None);
        while let Some(c) = curr {
            prev = Some(
                Box::new(ListNode {
                    next: prev,
                    val: c.val
                })
            );
            curr = c.next;
        }
        prev
    }
}