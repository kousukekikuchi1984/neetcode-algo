struct Solution {}

impl Solution {
    pub fn insert_sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.to_vec();
        for i in 1..nums.len() {
            let mut j = i;
            while j > 0 && nums[j - 1] > nums[j] {
                let current = nums[j];
                nums[j] = nums[j - 1];
                nums[j - 1] = current;
                j -= 1;
            }
        }
        return nums;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_insert_sort_array() {
        let lis = vec![5, 4, 3, 2, 1];
        let actual = Solution::insert_sort_array(lis);
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(actual, expected);
    }
}
