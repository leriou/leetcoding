impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let some_box_list = |val, next| Some(Box::new(ListNode { val, next }));

        match (l1, l2) {
            (Some(left_node), Some(right_node)) => if left_node.val < right_node.val {
                some_box_list(left_node.val, Self::merge_two_lists(left_node.next, Some(right_node)))
            } else {
                some_box_list(right_node.val, Self::merge_two_lists(Some(left_node), right_node.next))
            },
            (Some(left_node), _) => some_box_list(left_node.val, Self::merge_two_lists(left_node.next, None)),
            (l1, l2 @ Some(_)) => Self::merge_two_lists(l2, l1),
            _ => None
        }
    }
}