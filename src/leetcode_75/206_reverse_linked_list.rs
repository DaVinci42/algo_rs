type LeetNode = Option<Box<ListNode>>;

impl Solution {
    pub fn reverse_list(mut head: LeetNode) -> LeetNode {
        let mut prev: LeetNode = None;
        while let Some(mut current) = head {
            head = current.next.take();
            current.next = prev;
            prev = Some(current);
        }
        prev
    }
}
