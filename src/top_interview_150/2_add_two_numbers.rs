type LeetNode = Option<Box<ListNode>>;

impl Solution {
    pub fn add_two_numbers(l1: LeetNode, l2: LeetNode) -> LeetNode {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        let (mut l1, mut l2) = (l1, l2);
        let mut carry = 0;
        while carry != 0 || l1.is_some() || l2.is_some() {
            let mut cur = carry;
            if let Some(b) = l1 {
                cur += b.val;
                l1 = b.next;
            }
            if let Some(b) = l2 {
                cur += b.val;
                l2 = b.next;
            }
            carry = cur / 10;
            tail.next = Some(Box::new(ListNode::new(cur % 10)));
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }
}
