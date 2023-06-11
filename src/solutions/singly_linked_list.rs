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
        // 1. determine the partition by using fast and slow pointers
        let mut fast = head;
        let mut slow = head;
        while fast.is_some() && fast.next.is_some() {
            slow = slow.next;
            fast = fast.next;
            if fast.next.is_none() {
                // odd pattern
                slow = slow.next;
                break;
            } else {
                fast.next = fast;
            }
        }

        // 2. reverse the latter list.
        // start -> slow
        let mut start = slow;
        let mut current = start;
        while let Some(mut boxed_head) = current {
            current = &mut boxed_head.next.take();
            boxed_head.next = prev;
            prev = Some(boxed_head);
        }

        // 3. pick one by one.
        let mut normal = head;
        let mut normal_next = head.next;
        let mut reverse = current;
        let mut reverse_next = head.next;
        while reverse.is_some() {
            tmp_normal = normal.next;
            normal.next = reverse;
            reverse.next = normal_next;
            normal = normal_next;
            normal_next = tmp_normal;
            reverse = reverse_next;
        }
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
