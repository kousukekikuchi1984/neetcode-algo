use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];
        dp[0] = 0;
        dp[1] = nums[0];
        for i in 1..nums.len() {
            dp[i + 1] = max(dp[i], dp[i - 1] + nums[i]);
        }
        dp[nums.len()]
    }

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let r = (m - 1).min(n - 1) as i64;
        let other = (m + n - 2) as i64 - r;
        (1..=r).fold(1, |acc, x| acc * (x + other) / x) as i32
    }

    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        // ref: https://leetcode.com/problems/house-robber/
        let nums = vec![1, 2, 3, 1];
        let expected = 4;
        let actual = Solution::rob(nums);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_unique_paths() {
        // ref: https://leetcode.com/problems/unique-paths/
        let m = 3;
        let n = 2;
        let expected = 3;
        let actual = Solution::unique_paths(m, n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_unique_with_obstacles() {
        // ref: https://leetcode.com/problems/unique-paths-ii/
        let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let expected = 2;
        let actual = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(expected, actual);
    }
}
