/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public int kthToLast(ListNode head, int k) {
        ListNode a = head,b = head;
        while (b.next != null) {
            b = b.next;
            if (--k < 1) a = a.next;
        }
        return a.val;
    }
}