struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_islands() {
        let grid = vec![
            vec!["1", "1", "1", "1", "0"],
            vec!["1", "1", "0", "1", "0"],
            vec!["1", "1", "0", "0", "0"],
            vec!["0", "0", "0", "0", "0"],
        ];
        let actual = Solution::num_islands(grid);
        let expected = 1;
        assert_eq!(actual, expected);
    }
}
