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
}
