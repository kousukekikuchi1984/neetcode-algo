use std::mem;

struct Solution {}

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

// fn _reverse_list(
//     head: Option<Box<ListNode>>,
//     mut prev: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     if let Some(mut curr) = head {
//         let temp = curr.next.take();
//         curr.next = prev;
//         prev = Some(curr);
//         _reverse_list(temp, prev);
//     }
//     return prev;
// }

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

    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1 == None {
            return list2;
        }
        if list2 == None {
            return list1;
        }
        let mut dummy = None;
        let mut p_next = &mut dummy;
        while list1.is_some() && list2.is_some() {
            let l1 = &mut list1;
            let l2 = &mut list2;
            let l = if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                l1
            } else {
                l2
            };
            mem::swap(p_next, l);
            mem::swap(l, &mut p_next.as_mut().unwrap().next);
            p_next = &mut p_next.as_mut().unwrap().next;
        }
        mem::swap(
            p_next,
            if list1.is_none() {
                &mut list2
            } else {
                &mut list1
            },
        );
        dummy
    }

    // fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     _reverse_list(None, head)
    // }

    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None
        }
        let mut slow = &head;
        let mut fast = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_list() {
        Solution::reverse_list(None);
    }
}
