struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(Solution::three_sum(nums), expected);
    }
}
