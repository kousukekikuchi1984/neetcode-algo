struct Node {
    val: i32,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}

struct MyLinkedList {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
    length: i32,
}

impl MyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    //fn get(&mut self, index: i32) -> i32 {
    //    if let Some(ref cur) = self.get_node(index) {
    //        return cur.val;
    //    }
    //    -1
    //}

    //fn add_at_head(&mut self, val: i32) {
    //    self.add_at_index(0, val);
    //}

    //fn add_at_tail(&mut self, val: i32) {
    //    self.add_at_index(self.length, val);
    //}

    // fn add_at_index(&mut self, index: i32, val: i32) {
    //     let cur = self.get_node(index);
    //     if let Some(ref mut cur) = cur {
    //         let node = Some(Box::new(Node {
    //             val,
    //             next: cur.next.take(),
    //             prev: Some(*cur),
    //         }));
    //         cur.next = node;
    //         self.length += 1;
    //     }
    // }

    //fn delete_at_index(&mut self, index: i32) {
    //    let cur = self.get_node(index);
    //    if let Some(node) = cur {
    //        let to_delete = node.next.take();
    //        if let Some(mut to_delete) = to_delete {
    //            to_delete.prev = Some(*node);
    //            node.next = to_delete.next.take();
    //            self.length -= 1;
    //        }
    //    }
    //}

    //fn get_node(&mut self, with: i32) -> &mut Option<Box<Node>> {
    //    let mut cur = &mut self.head;
    //    for _ in 0..=with {
    //        if cur.is_none() {
    //            return cur;
    //        }
    //        cur = &mut cur.as_mut().unwrap().next;
    //    }
    //    cur
    //}
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

#[cfg(test)]
mod tests {
    use super::MyLinkedList;

    #[test]
    #[ignore]
    fn test_my_linked_list_new() {
        MyLinkedList::new();
    }
}
