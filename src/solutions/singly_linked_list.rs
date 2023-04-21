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
            return None;
        }
        let mut slow = &head;
        let mut fast = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow.clone()
    }

    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        return 0;
        // wip: fix error of borrow checker
        // the length of head is even
        // fast-slow pointer
        let mut slow: &Option<Box<ListNode>> = &head;
        let mut fast: &Option<Box<ListNode>> = &head;
        let mut prev: &Option<Box<ListNode>> = &None;
        let mut num = 0;
        // reverse slow
        while fast.is_some() && fast.unwrap().next.is_some() {
            slow = &slow.unwrap().next;
            let temp = slow.unwrap().next.take();
            slow.unwrap().next = prev.take();
            prev = slow;
            fast = &fast.unwrap().next.unwrap().next;
        }
        // fast -> out of bound
        // slow -> from middle to end
        // prev -> from start to middle
        while slow.is_some() && prev.is_some() {
            // compare num and slow.val + prev.val
            num = num.max(slow.as_ref().unwrap().val + prev.as_ref().unwrap().val);
            slow = &slow.as_ref().unwrap().next;
            prev = &prev.as_ref().unwrap().next;
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;
    use super::Solution;

    #[test]
    fn test_list() {
        Solution::reverse_list(None);
    }

    #[test]
    #[ignore]
    fn test_pair_sum() {
        let mut list = ListNode::new(1);
        list.next = Some(Box::new(ListNode::new(2)));
        list.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        list.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        list.next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(5)));
        list.next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(6)));
        list.next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(7)));
        assert_eq!(Solution::pair_sum(Some(Box::new(list))), 7);
    }
}
