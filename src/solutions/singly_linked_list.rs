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

    pub fn create_linked_list(nums: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;

        for &val in nums {
            let new_node = Some(Box::new(ListNode::new(val)));
            *current = new_node;
            if let Some(node) = current {
                current = &mut node.next;
            }
        }

        head
    }

    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        fn get_list_middle(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let (mut fast, mut slow) = (&head.clone(), head);
            while fast.is_some() {
                fast = &(fast.as_ref().unwrap().next);
                if fast.is_some() {
                    fast = &fast.as_ref().unwrap().next;
                    slow = &mut (slow.as_mut().unwrap().next);
                }
            }
            slow.as_mut().unwrap().next.take()
        }

        fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            while let Some(mut curr) = head {
                head = curr.next;
                curr.next = prev;
                prev = Some(curr);
            }
            prev
        }

        fn merge_lists(mut head1: &mut Option<Box<ListNode>>, mut head2: Option<Box<ListNode>>) {
            let mut first = head1;
            let mut second = head2;
            while first.is_some() && second.is_some() {
                let mut first_next = first.as_mut().unwrap().next.take();
                let mut second_next = second.as_mut().unwrap().next.take();
                first.as_mut().unwrap().next = second;
                first.as_mut().unwrap().next.as_mut().unwrap().next = first_next;
                first = &mut (first.as_mut().unwrap().next.as_mut().unwrap().next);
                second = second_next;
            }
            if second.is_some() {
                first = &mut (second);
            }
        }

        let mut head2 = get_list_middle(head);
        head2 = reverse_list(head2);
        merge_lists(head, head2);
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
    fn test_reorder_list() {
        let nums = [1, 2, 3, 4, 5];
        let mut head = Solution::create_linked_list(&nums);
        Solution::reorder_list(&mut head);
        let mut expected = Solution::create_linked_list(&[1, 5, 2, 4, 3]);
        assert_eq!(head, expected);
    }
}
