struct Solution {}

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ans = nums.clone();
        ans.append(&mut nums);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_concatenation() {
        Solution::get_concatenation(vec![1, 2, 3]);
    }
}
