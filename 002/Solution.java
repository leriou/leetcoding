/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        if (l1 == null && l2 == null) return null;
        ListNode first = l1;
        ListNode second = l2;
        int carry = 0;
        ListNode head = new ListNode(0);
        ListNode prev = head;
        while (first != null && second != null) {
            int q = (first.val + second.val + carry) / 10;
            int r = (first.val + second.val + carry) % 10;
            carry = q;
            ListNode node = new ListNode(r);
            prev.next = node;
            prev = node;
            first = first.next;
            second = second.next;
        }

        while (first != null) {
            int q = (first.val + carry) / 10;
            int r = (first.val + carry) % 10;
            carry = q;
            ListNode node = new ListNode(r);
            prev.next = node;
            prev = node;
            first = first.next;
        }

        while (second != null) {
            int q = (second.val + carry) / 10;
            int r = (second.val + carry) % 10;
            carry = q;
            ListNode node = new ListNode(r);
            prev.next = node;
            prev = node;
            second = second.next;
        }

        if (carry != 0) prev.next = new ListNode(carry);

        return head.next;
    }
}