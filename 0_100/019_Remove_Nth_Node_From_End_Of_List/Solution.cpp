/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode* dummy = new ListNode(0);
        dummy->next = head;
        ListNode* a = dummy, *b = head;
        while (b) {
            b = b->next;
            if (n-- <= 0)
                a = a->next;
        }
        a->next = a->next->next;
        return dummy->next;
    }
};