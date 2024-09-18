impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        let mut node = &head;
        while let Some(b) = &node {
            let next_val = &b.next.as_ref().map(|b| b.val).unwrap_or(i32::MAX);
            if &b.val != next_val {
                tail.next = Some(Box::new(ListNode::new(b.val)));
                tail = tail.next.as_mut().unwrap();
                node = &b.next;
            } else {
                while let Some(b) = &node {
                    if &b.val == next_val {
                        node = &b.next;
                    } else {
                        break;
                    }
                }
            }
        }
        dummy.next
    }
}
