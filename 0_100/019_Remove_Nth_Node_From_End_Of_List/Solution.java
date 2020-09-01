/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public ListNode removeNthFromEnd(ListNode head, int n) {
        ListNode b = head, dummy = new ListNode(0);
        dummy.next = head;
        ListNode a = dummy;
        while (b != null) {
            b = b.next;
            if (n-- <= 0)
                a = a.next;
        }
        a.next = a.next.next;
        return dummy.next;
    }
}