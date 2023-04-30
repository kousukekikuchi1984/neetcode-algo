struct Solution {}

impl Solutions {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_redundant_connection() {
        let edges = vec![vec![1,2], vec![1,3], vec![2,3]];
        let expected = vec![2,3];
        assert_eq!(Solution::find_redundant_connection(edges), expected);
    }
}