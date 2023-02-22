// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut head) = (None, head);
        while let Some(mut node) = head {
            head = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    pub fn reverse_list_recursive(&self, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        self._reverse_list(None, head)
    }

    pub fn _reverse_list(
        &self,
        prev: Option<Box<ListNode>>,
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut next = None;

        if cur.is_some() {
            let mut n = cur.take().unwrap();
            next = n.next.take();
            n.next = prev;
            self._reverse_list(Some(n), next);
        }
        prev
    }
}
