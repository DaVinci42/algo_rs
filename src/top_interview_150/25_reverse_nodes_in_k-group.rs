impl Solution {
    fn reverse(nums: &mut [i32], mut i: usize, mut j: usize) {
        while i < j {
            (nums[i], nums[j]) = (nums[j], nums[i]);
            (i, j) = (i + 1, j - 1);
        }
    }

    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let (mut v, mut cur) = (vec![], &head);
        while let Some(b) = cur {
            v.push(b.val);
            cur = &b.next;
        }

        let (mut i, k) = (0, k as usize);
        while i + k - 1 < v.len() {
            Self::reverse(&mut v, i, i + k - 1);
            i += k;
        }

        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        for &n in v.iter() {
            tail.next = Some(Box::new(ListNode::new(n)));
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }
}
