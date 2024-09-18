impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left_dummy = ListNode::new(0);
        let mut left_tail = &mut left_dummy;

        let mut right_dummy = ListNode::new(0);
        let mut right_tail = &mut right_dummy;

        let mut node = &head;
        while let Some(b) = node {
            let cur = Some(Box::new(ListNode::new(b.val)));
            if b.val < x {
                left_tail.next = cur;
                left_tail = left_tail.next.as_mut().unwrap();
            } else {
                right_tail.next = cur;
                right_tail = right_tail.next.as_mut().unwrap();
            }
            node = &b.next;
        }
        if right_dummy.next.is_some() {
            left_tail.next = right_dummy.next;
        }
        left_dummy.next
    }
}
