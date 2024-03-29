use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = vec![];
        let mut checked = 0;
        let end = 1 << nums.len();
        while checked < end {
            let mut result: Vec<i32> = vec![];
            for i in 0..nums.len() {
                if checked & (1 << i) != 0 {
                    result.push(nums[i]);
                }
            }
            results.push(result);
            checked += 1;
        }
        results
    }

    pub fn subsets_tree(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(nums: &[i32], index: usize, results: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>) {
            if index >= nums.len() {
                results.push(subset.to_vec());
                return;
            }
            subset.push(nums[index]);
            dfs(nums, index + 1, results, subset);
            subset.pop();
            dfs(nums, index + 1, results, subset);
        }

        let mut results: Vec<Vec<i32>> = vec![];
        let mut subset: Vec<i32> = vec![];
        dfs(&nums, 0, &mut results, &mut subset);
        results
    }

    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(
            candidates: &mut Vec<i32>,
            index: usize,
            target: i32,
            results: &mut Vec<Vec<i32>>,
            subset: &mut Vec<i32>,
        ) {
            if target == 0 {
                results.push(subset.to_vec());
                return;
            }
            if target < 0 {
                return;
            }
            // we can use a candidate multiple times
            for i in index..candidates.len() {
                subset.push(candidates[i]);
                dfs(candidates, i, target - candidates[i], results, subset);
                subset.pop();
            }
        }

        let mut results: Vec<Vec<i32>> = vec![];
        dfs(&mut candidates, 0, target, &mut results, &mut vec![]);
        return results;
    }

    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        // heap must contain at least 2 stones
        while heap.len() > 1 {
            // get two stones from heap
            let x = heap.pop().unwrap();
            let y = heap.pop().unwrap();
            match x - y {
                // if they are equal, delete then = do nothing
                0 => {}
                // if they are not equal, the difference should be back to the queue
                diff => heap.push(diff.abs()),
            }
        }
        match heap.pop() {
            None => 0,
            Some(v) => v,
        }
    }

    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        for point in points {
            // no need to calculate square root of distance
            let distance = point[0] * point[0] + point[1] * point[1];
            heap.push((distance, point));
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        heap.into_vec()
            .into_iter()
            .map(|(_, point)| point)
            .collect()
    }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(nums);
        let mut result = -1;
        let mut k = k - 1;
        while let Some(val) = heap.pop() {
            if k == 0 {
                result = val;
            }
            k -= 1;
        }
        result
    }

    pub fn find_min(nums: Vec<i32>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::with_capacity(nums.len());
        for num in nums {
            heap.push(Reverse(num));
        }
        heap.pop().unwrap().0
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if target == nums[mid] {
                return mid as i32;
            }
            if nums[left] <= nums[mid] {
                if target < nums[left] || target > nums[mid] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            } else {
                if target < nums[mid] || target > nums[right] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
        }
        -1
    }
}

struct KthLargest {
    nums: Vec<i32>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        KthLargest {
            nums,
            k: k as usize,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(val);
        self.nums.sort();
        self.nums[self.nums.len() - self.k as usize]
    }
}
struct KthLargestHeap {
    heap: BinaryHeap<i32>,
    k: usize,
}

impl KthLargestHeap {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self {
            heap: BinaryHeap::from(nums),
            k: k as usize,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(val);
        let mut local_heap = self.heap.clone();
        let mut result = -1;
        for _ in 0..self.k {
            match local_heap.pop() {
                None => {
                    result = -1;
                    break;
                }
                Some(v) => result = v,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        let nums = vec![1, 2, 3];
        let actual = Solution::subsets(nums);
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_combination_sub() {
        let nums = vec![2, 3, 6, 7];
        let actual = Solution::combination_sum(nums, 7);
        let expected = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_kth_largest() {
        let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);
    }

    #[test]
    fn test_kth_largest_heap() {
        let mut kth_largest = KthLargestHeap::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);
    }

    #[test]
    fn test_last_stone_weight() {
        let stones = vec![2, 7, 4, 1, 8, 1];
        let actual = Solution::last_stone_weight(stones);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_k_closest() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let actual = Solution::k_closest(points, 1);
        let expected = vec![vec![-2, 2]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_kth_largest() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let actual = Solution::find_kth_largest(nums, 2);
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_find_min() {
        let nums = vec![3, 4, 5, 1, 2];
        let actual = Solution::find_min(nums);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_search() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let actual = Solution::search(nums, 0);
        let expected = 4;
        assert_eq!(actual, expected);
    }
}
