use std::cmp::max;
use std::cmp::Ordering;

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
        // using Dynamic Programming
        let mut dp = vec![vec![0; obstacle_grid[0].len()]; obstacle_grid.len()];
        dp[0][0] = 1;
        for i in 0..obstacle_grid.len() {
            for j in 0..obstacle_grid[i].len() {
                match obstacle_grid[i][j] {
                    1 => dp[i][j] = 0,
                    _ => {
                        if i > 0 {
                            dp[i][j] += dp[i - 1][j];
                        }
                        if j > 0 {
                            dp[i][j] += dp[i][j - 1];
                        }
                    }
                }
            }
        }
        dp[obstacle_grid.len() - 1][obstacle_grid[0].len() - 1]
    }

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        // using Dynamic Programming
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for i in 0..text1.len() {
            for j in 0..text2.len() {
                match text1
                    .chars()
                    .nth(i)
                    .unwrap()
                    .cmp(&text2.chars().nth(j).unwrap())
                {
                    Ordering::Equal => dp[i + 1][j + 1] = dp[i][j] + 1,
                    _ => dp[i + 1][j + 1] = max(dp[i][j + 1], dp[i + 1][j]),
                }
            }
        }

        dp[text1.len()][text2.len()]
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

    #[test]
    fn test_longest_common_subsequence() {
        // ref: https://leetcode.com/problems/longest-common-subsequence/
        let text1 = "abcde".to_string();
        let text2 = "ace".to_string();
        let expected = 3;
        let actual = Solution::longest_common_subsequence(text1, text2);
        assert_eq!(expected, actual);
    }
}
