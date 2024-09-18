use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        let mut cur = head;
        while let Some(b) = cur {
            heap.push(Reverse(b.val));
            cur = b.next;
        }

        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;
        while let Some(val) = heap.pop() {
            let node = ListNode::new(val.0);
            cur.next = Some(Box::new(node));
            cur = cur.next.as_mut().unwrap();
        }
        dummy.next
    }
}
