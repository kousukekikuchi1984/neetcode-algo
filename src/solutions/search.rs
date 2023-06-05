use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

struct Solution {}

fn guess(num: i32) -> i32 {
    if num > 1150769282 {
        return -1;
    }

    if num < 1150769282 {
        return 1;
    }

    0
}

fn isBadVersion(version: i32) -> bool {
    version >= 1150769282
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        fn _search(nums: &[i32], left: isize, right: isize, target: i32) -> isize {
            if left > right {
                return -1;
            }
            let middle = left + (right - left) / 2;
            match target.cmp(&nums[middle as usize]) {
                Ordering::Less => _search(nums, left, middle - 1, target),
                Ordering::Equal => middle,
                Ordering::Greater => _search(nums, middle + 1, right, target),
            }
        }

        let length = nums.len() - 1;
        let num = _search(&nums, 0, length.try_into().unwrap(), target);
        return num as i32;
    }

    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        fn _search_column(matrix: &[&[i32]], bottom: isize, top: isize, target: i32) -> isize {
            let row = (top + bottom) / 2;
            let last = matrix[0].len() - 1;

            if bottom > top || bottom < 0 || top > (matrix.len() - 1) as isize {
                return -1;
            }
            if matrix[row as usize][last] < target {
                return _search_column(matrix, row + 1, top, target);
            } else if matrix[row as usize][0] > target {
                return _search_column(matrix, bottom, row - 1, target);
            } else {
                return row;
            }
        }

        fn _search_rows(rows: &[i32], left: isize, right: isize, target: i32) -> bool {
            if left > right {
                return false;
            }
            let middle = (left + right) / 2;
            match target.cmp(&rows[middle as usize]) {
                Ordering::Less => _search_rows(rows, left, middle - 1, target),
                Ordering::Equal => true,
                Ordering::Greater => _search_rows(rows, middle + 1, right, target),
            }
        }
        let rows: Vec<&[i32]> = matrix.iter().map(|v| v.as_slice()).collect();
        let row = _search_column(rows.as_slice(), 0, (matrix.len() - 1) as isize, target);
        if row == -1 {
            return false;
        }
        let rows = &matrix[row as usize];
        _search_rows(&rows, 0, (rows.len() - 1) as isize, target)
    }

    unsafe fn guessNumber(n: i32) -> i32 {
        let mut start = 0;
        let mut end = n;
        while start <= end {
            let middle = start + (end - start) / 2;
            match guess(middle) {
                -1 => {
                    end = middle - 1;
                }
                0 => return middle,
                1 => {
                    start = middle + 1;
                }
                _ => panic!("unreachable"),
            };
        }
        return -1;
    }

    pub fn first_bad_version(n: i32) -> i32 {
        let mut start = 1;
        let mut end = n;
        while start < end {
            let middle = start + (end - start) / 2;
            match isBadVersion(middle) {
                true => end = middle,
                false => start = middle + 1,
            };
        }
        return start;
    }

    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut left, mut right) = (1, *piles.iter().max().unwrap());
        while left < right {
            let mid = left + (right - left) / 2;
            let time = piles.iter().map(|&p| (p + mid - 1) / mid).sum::<i32>();
            if time <= h {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }

    pub fn search_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = root;
        while let Some(n) = node {
            let cmp = n.borrow().val.cmp(&val);
            if cmp == Ordering::Equal {
                return Some(n);
            }
            node = match cmp {
                Ordering::Greater => n.borrow().left.clone(),
                Ordering::Less => n.borrow().right.clone(),
                _ => unreachable!(),
            };
        }
        None
    }

    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(match root {
            None => Rc::new(RefCell::new(TreeNode::new(val))),
            Some(cur) => {
                if cur.borrow().val > val {
                    let node = Solution::insert_into_bst(cur.borrow().left.clone(), val);
                    cur.borrow_mut().left = node;
                } else {
                    let node = Solution::insert_into_bst(cur.borrow().right.clone(), val);
                    cur.borrow_mut().right = node;
                }
                cur
            }
        })
    }

    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn _delete_node(
            node: &Option<Rc<RefCell<TreeNode>>>,
            key: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(n) = node {
                let val = n.borrow().val;
                match val.cmp(&key) {
                    std::cmp::Ordering::Greater => {
                        let l = _delete_node(&n.borrow().left, key);
                        n.borrow_mut().left = l;
                    }
                    std::cmp::Ordering::Less => {
                        let r = _delete_node(&n.borrow().right, key);
                        n.borrow_mut().right = r;
                    }
                    std::cmp::Ordering::Equal => {
                        if n.borrow().left.is_none() {
                            return n.borrow().right.clone();
                        }
                        if n.borrow().right.is_none() {
                            return n.borrow().left.clone();
                        }
                        let next = _search_next(&n.borrow().right);
                        if let Some(val) = next {
                            let r = _delete_node(&n.borrow().right, val);
                            n.borrow_mut().val = val;
                            n.borrow_mut().right = r;
                        }
                    }
                }
            }
            node.clone()
        }

        fn _search_next(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            if let Some(n) = node {
                if n.borrow().left.is_some() {
                    _search_next(&n.borrow().left)
                } else {
                    Some(n.borrow().val)
                }
            } else {
                None
            }
        }

        _delete_node(&root, key)
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn _inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>, results: &mut Vec<i32>) {
            match root {
                None => {}
                Some(cur) => {
                    _inorder_traversal(cur.borrow().left.clone(), results);
                    results.push(cur.borrow().val);
                    _inorder_traversal(cur.borrow().right.clone(), results);
                }
            }
        }
        let mut results: Vec<i32> = vec![];
        _inorder_traversal(root, &mut results);
        results
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        fn _kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> i32 {
            // use inorder traversal
            if let Some(cur) = root {
                let left = _kth_smallest(cur.borrow().left.clone(), k);
                if *k == 0 {
                    return left;
                }
                *k -= 1;
                if *k == 0 {
                    return cur.borrow().val;
                }
                let right = _kth_smallest(cur.borrow().right.clone(), k);
                if *k == 0 {
                    return right;
                }
            }
            0
        }

        _kth_smallest(root, &mut k)
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn _build_tree(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() || inorder.is_empty() {
                return None;
            }
            let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
            let mid = inorder.iter().position(|&x| x == preorder[0]).unwrap();
            root.borrow_mut().left = _build_tree(&preorder[1..mid + 1], &inorder[..mid]);
            root.borrow_mut().right = _build_tree(&preorder[mid + 1..], &inorder[mid + 1..]);
            Some(root)
        }

        _build_tree(&preorder, &inorder)
    }
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn _level_order(
            root: Option<Rc<RefCell<TreeNode>>>,
            mut level: usize,
            results: &mut Vec<Vec<i32>>,
        ) {
            match root {
                None => {}
                Some(cur) => {
                    if level >= results.len() {
                        results.push(vec![]);
                    }
                    results[level].push(cur.borrow().val);
                    _level_order(cur.borrow().left.clone(), level + 1, results);
                    _level_order(cur.borrow().right.clone(), level + 1, results);
                }
            }
        }

        let mut results: Vec<Vec<i32>> = vec![];
        _level_order(root, 0, &mut results);
        return results;
    }

    pub fn level_order_bfs(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = vec![];
        let mut queue: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        queue.push(root.clone());
        while !queue.is_empty() {
            let mut level = vec![];
            let mut next_level = vec![];
            for node in queue {
                if let Some(n) = node {
                    level.push(n.borrow().val);
                    next_level.push(n.borrow().left.clone());
                    next_level.push(n.borrow().right.clone());
                }
            }
            if !level.is_empty() {
                results.push(level);
            }
            queue = next_level;
        }

        results
    }

    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut results: Vec<i32> = vec![];
        let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![];
        if root.is_some() {
            queue.push(root.clone().unwrap());
            results.push(root.clone().unwrap().borrow().val);
        }
        while !queue.is_empty() {
            let mut next_level: Vec<Rc<RefCell<TreeNode>>> = vec![];
            for node in queue {
                if let n = node {
                    let left = n.borrow().left.clone();
                    if let Some(l) = left {
                        next_level.push(l);
                    }
                    let right = n.borrow().right.clone();
                    if let Some(r) = n.borrow().right.clone() {
                        next_level.push(r);
                    }
                }
            }
            let len = next_level.len();
            if len > 0 {
                results.push(next_level[len - 1].borrow().val);
            }
            queue = next_level;
        }

        results
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn _has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, sum: i32) -> bool {
            return match root {
                None => false,
                Some(cur) => {
                    let sum = sum + cur.borrow().val;
                    if cur.borrow().left.is_none() && cur.borrow().right.is_none() {
                        return sum == target_sum;
                    }
                    _has_path_sum(cur.borrow().left.clone(), target_sum, sum)
                        || _has_path_sum(cur.borrow().right.clone(), target_sum, sum)
                }
            };
        }

        _has_path_sum(root, target_sum, 0)
    }
}

struct TimeMap {}

impl TimeMap {
    fn new() -> Self {}

    fn set(&self, key: String, value: String, timestamp: i32) {}

    fn get(&self, key: String, timestamp: i32) -> String {}
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;
    use crate::solutions::search::TimeMap;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_binary_search() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let actual = Solution::binary_search(nums, 9);
        assert_eq!(actual, 4);
        assert_eq!(Solution::binary_search(vec![5], -5), -1);
    }

    #[test]
    fn test_search_matrix() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let actual = Solution::search_matrix(matrix, 3);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_guess() {
        unsafe {
            let actual = Solution::guessNumber(1420736637);
            assert_eq!(actual, 1150769282);
        }
    }

    #[test]
    fn test_first_bad_version() {
        let actual = Solution::first_bad_version(1420736637);
        assert_eq!(actual, 1150769282);
    }

    #[test]
    fn test_min_eating_time() {
        let actual = Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5);
        assert_eq!(actual, 30);
        let actual = Solution::min_eating_speed(vec![312884470], 312884469);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_kth_smallest() {
        let actual = Solution::kth_smallest(None, 1);
        assert_eq!(actual, 0);
        let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let actual = Solution::kth_smallest(root, 1);
        assert_eq!(actual, 3);
        let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        if let Some(node) = &root {
            node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
            node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        }
        let actual = Solution::kth_smallest(root.clone(), 3);
        assert_eq!(actual, 6);
    }

    #[test]
    fn test_level_order() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        if let Some(node) = &root {
            node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
            node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        }
        let actual = Solution::level_order(root);
        assert_eq!(actual, vec![vec![3], vec![9, 20]]);
    }

    #[test]
    fn test_level_order_bfs() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        if let Some(node) = &root {
            node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
            node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        }
        let actual = Solution::level_order_bfs(root);
        assert_eq!(actual, vec![vec![3], vec![9, 20]]);
    }

    #[test]
    fn test_right_side_view() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        if let Some(node) = &root {
            node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
            node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        }
        let actual = Solution::right_side_view(root);
        assert_eq!(actual, vec![1, 3]);
    }

    #[test]
    fn test_has_path_sum() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        if let Some(node) = &root {
            node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(8))));
            node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        }
        let actual = Solution::has_path_sum(root, 9);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_time_map() {
        let mut map = TimeMap::new();
        map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(map.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(map.get("foo".to_string(), 3), "bar".to_string());
        map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(map.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(map.get("foo".to_string(), 5), "bar2".to_string());
    }
}
