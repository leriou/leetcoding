func reverseList(head *ListNode) *ListNode {
    if head == nil || head.Next == nil {
        return head
    }
    newList := reverseList(head.Next)
    head.Next.Next = head
    head.Next = nil
    return newList
}
