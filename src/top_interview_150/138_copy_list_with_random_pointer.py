class Solution:
    def copyRandomList(self, head: "Optional[Node]") -> "Optional[Node]":
        if not head:
            return None

        mapping, cur = {None: None}, head
        while cur:
            mapping[cur] = Node(cur.val)
            cur = cur.next

        cur = head
        while cur:
            mapping[cur].next = mapping[cur.next]
            mapping[cur].random = mapping[cur.random]
            cur = cur.next

        return mapping[head]
