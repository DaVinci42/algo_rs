class Solution:
    def construct(self, grid: List[List[int]]) -> "Node":

        def dfs(r: int, c: int, size: int) -> "Node":
            if size == 1:
                return Node(grid[r][c], True, None, None, None, None)
            else:
                size //= 2
                children = [
                    dfs(r, c, size),
                    dfs(r, c + size, size),
                    dfs(r + size, c, size),
                    dfs(r + size, c + size, size),
                ]
                if all(x.isLeaf and x.val == children[0].val for x in children):
                    return Node(grid[r][c], True, None, None, None, None)
                else:
                    return Node(0, False, children[0], children[1], children[2], children[3])

        return dfs(0, 0, len(grid))
