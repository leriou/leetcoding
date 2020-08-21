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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut _list = ListNode { val: 0, next: None };
        let mut _tail = &mut _list;
        let mut _carry_digit: i32 = 0;
        let mut _a1 = &l1;
        let mut _a2 = &l2;
        loop {
            let mut _stop = true;
            let mut add: i32 = _carry_digit;
            if let Some(n1) = _a1 {
                add += n1.val;
                _a1 = &n1.next;
                _stop = false;
            }
            if let Some(n2) = _a2 {
                add += n2.val;
                _a2 = &n2.next;
                _stop = false;
            }
            if _stop == true && add == 0 {
                break;
            }
            _carry_digit = add / 10;
            _tail.next = Some(Box::new(ListNode::new(add % 10)));
            _tail = _tail.next.as_mut().unwrap();
        }
        return _list.next;
    }
}

