impl Solution {
    fn reverse(nums: &mut [i32], mut i: usize, mut j: usize) {
        while i < j {
            (nums[i], nums[j]) = (nums[j], nums[i]);
            (i, j) = (i + 1, j - 1);
        }
    }

    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut v = vec![];
        while let Some(b) = head {
            v.push(b.val);
            head = b.next;
        }
        Self::reverse(&mut v, left as usize - 1, right as usize - 1);
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        for &n in v.iter() {
            tail.next = Some(Box::new(ListNode::new(n)));
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }
}
