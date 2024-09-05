type LeetNode = Option<Box<ListNode>>;

impl Solution {
    pub fn pair_sum(head: LeetNode) -> i32 {
        let (mut v, mut cur) = (vec![], &head);
        while let Some(b) = cur {
            v.push(b.val);
            cur = &b.next;
        }

        (0..v.len() / 2)
            .map(|i| v[i] + v[v.len() - 1 - i])
            .max()
            .unwrap_or(0)
    }
}
