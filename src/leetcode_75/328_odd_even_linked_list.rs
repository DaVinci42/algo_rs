impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut vec, mut cur) = (vec![], &head);
        while let Some(b) = cur {
            vec.push(b.val);
            cur = &b.next;
        }
        vec = (0..vec.len())
            .step_by(2)
            .chain((1..vec.len()).step_by(2))
            .map(|i| vec[i])
            .collect();

        let mut cur = &mut head;
        for &n in vec.iter() {
            let b = cur.as_mut().unwrap();
            b.val = n;
            cur = &mut b.next;
        }
        head
    }
}
