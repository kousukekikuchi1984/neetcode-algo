struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        // leetcode: https://leetcode.com/problems/house-robber/
        // example 1
        let nums = vec![1, 2, 3, 1];
        let expected = 4;
        let actual = Solution::rob(nums);
        assert_eq!(expected, actual);
    }
}