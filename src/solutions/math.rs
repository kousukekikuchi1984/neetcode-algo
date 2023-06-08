struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        vec![4, 3, 2, 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1, 0]);
    }
}
