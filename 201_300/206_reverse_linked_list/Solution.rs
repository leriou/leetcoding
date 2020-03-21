impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut prev = None;
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
