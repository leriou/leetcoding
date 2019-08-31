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
    bool hasCycle(ListNode *head) {
        for(ListNode *fast = head, *slow = head; fast && fast->next;)
            if((fast = fast->next->next) == (slow = slow->next))
                return true;
        return false;
    }
};