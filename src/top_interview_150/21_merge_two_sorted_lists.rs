impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        let (mut l1, mut l2) = (list1, list2);
        loop {
            let mut val = i32::MIN;
            match (&mut l1, &mut l2) {
                (Some(b1), Some(b2)) => {
                    if b1.val <= b2.val {
                        val = b1.val;
                        l1 = b1.next.take();
                    } else {
                        val = b2.val;
                        l2 = b2.next.take();
                    }
                }
                (Some(b1), None) => {
                    val = b1.val;
                    l1 = b1.next.take();
                }
                (None, Some(b2)) => {
                    val = b2.val;
                    l2 = b2.next.take();
                }
                (None, None) => break,
            }
            let node = Some(Box::new(ListNode::new(val)));
            tail.next = node;
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }
}
