impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut fast = &head.clone();
        for _ in 0..n {
            fast = &fast.as_ref().unwrap().next;
        }
        if fast.is_none() {
            return head.unwrap().next;
        }

        let mut slow = &mut head;
        while fast.as_ref().unwrap().next.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }

        let node = slow.as_ref().unwrap().next.as_ref().unwrap().next.clone();
        slow.as_mut().unwrap().next = node;

        head
    }
}
