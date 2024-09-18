class Solution:
    def cloneGraph(self, node: Optional["Node"]) -> Optional["Node"]:
        if not node:
            return None

        mapping = {}

        def dfs(node: Node) -> None:
            if node in mapping:
                return
            mapping[node] = Node(node.val)
            for n in node.neighbors:
                dfs(n)

        dfs(node)

        def dfsRelation(node: Node) -> None:
            if mapping[node].neighbors:
                return
            mapping[node].neighbors = [mapping[n] for n in node.neighbors]
            for n in node.neighbors:
                dfsRelation(n)

        dfsRelation(node)
        return mapping[node]
