class Solution:
    def connect(self, root: "Node") -> "Node":
        if not root:
            return root
        deque = Deque([root])
        while deque:
            for i in range(1, len(deque)):
                deque[i - 1].next = deque[i]
            for _ in range(len(deque)):
                node = deque.popleft()
                if node.left:
                    deque += [node.left]
                if node.right:
                    deque += [node.right]
        return root
