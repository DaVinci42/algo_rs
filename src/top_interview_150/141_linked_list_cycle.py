class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        if not head:
            return False
        slow, fast = head, head.next
        while True:
            if not fast or not fast.next:
                return False
            if slow == fast:
                return True
            slow = slow.next
            fast = fast.next.next
