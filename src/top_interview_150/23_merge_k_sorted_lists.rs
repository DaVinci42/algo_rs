use std::{
    cmp::{self, Reverse},
    collections::BinaryHeap,
};

impl cmp::PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for ListNode {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<_> = lists
            .iter()
            .filter_map(|n| n.as_ref().map(|b| Reverse((b.val, n))))
            .collect();

        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        while let Some(Reverse((val, node))) = heap.pop() {
            let cur = Some(Box::new(ListNode::new(val)));
            tail.next = cur;
            tail = tail.next.as_mut().unwrap();
            if let Some(b) = &node.as_ref().unwrap().next {
                heap.push(Reverse((b.val, &node.as_ref().unwrap().next)));
            }
        }
        dummy.next
    }
}
