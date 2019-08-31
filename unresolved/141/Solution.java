public class Solution {
    public boolean hasCycle(ListNode head) {
        for (ListNode fast = head, slow = head; fast != null && fast.next != null;)
            if ((fast = fast.next.next) == (slow = slow.next))
                return true;
        return false;
    }
}