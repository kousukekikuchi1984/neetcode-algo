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
}

struct KthLargest {
    nums: Vec<i32>,
    k: i32,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {}

    fn add(&self, val: i32) -> i32 {}
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

    fn test_combination_sub() {
        let nums = vec![2, 3, 6, 7];
        let actual = Solution::combination_sum(nums, 7);
        let expected = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(actual, expected);

        let nums = vec![2, 3, 6, 7];
        let actual = Solution::combination_sum(nums, 8);
        assert_eq!(actual, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }

    fn test_kth_largest() {
        let kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);
    }
}
