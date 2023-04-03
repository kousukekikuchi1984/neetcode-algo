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
}