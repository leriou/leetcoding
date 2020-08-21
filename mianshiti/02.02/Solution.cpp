/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution
{
public:
    int kthToLast(ListNode *head, int k)
    {
        ListNode *a = head, *b = head;
        while (b->next)
        {
            b = b->next;
            if (--k < 1)
                a = a->next;
        }
        return a->val;
    }
};