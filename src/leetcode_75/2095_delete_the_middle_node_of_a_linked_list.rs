impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?.next.as_ref()?;

        let (mut cur, mut length) = (&head, 0);
        while let Some(b) = cur {
            length += 1;
            cur = &b.next;
        }

        let mut cur = &mut head;
        for _ in 0..length / 2 - 1 {
            if let Some(b) = cur {
                cur = &mut b.next;
            }
        }
        if let Some(b) = cur {
            b.next = b.next.as_mut()?.next.take();
        }
        head
    }
}
