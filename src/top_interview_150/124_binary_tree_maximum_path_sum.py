class Solution:

    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        @cache
        def maxPath(node: TreeNode) -> int:
            if not node:
                return 0
            return max(
                node.val,
                node.val + max(maxPath(node.left), maxPath(node.right)),
            )

        res = -maxsize

        def dfs(node: TreeNode | None):
            if not node:
                return
            leftPath, rightPath = maxPath(node.left), maxPath(node.right)
            nonlocal res
            res = max(res, max(0, leftPath) + node.val + max(0, rightPath))
            dfs(node.left)
            dfs(node.right)

        dfs(root)
        return res
