impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let (mut cur, mut len) = (&head, 0);
        while let Some(node) = cur {
            len += 1;
            cur = &node.next;
        }

        k %= len;
        if k == 0 {
            return head;
        }

        let mut node = head.as_deref_mut().unwrap();
        for _ in 0..(len - k - 1) {
            node = node.next.as_deref_mut().unwrap();
        }

        let mut fin_head = node.next.take().unwrap();
        node = fin_head.as_mut();
        while node.next.is_some() {
            node = node.next.as_deref_mut().unwrap();
        }
        node.next = head;
        Some(fin_head)
    }
}