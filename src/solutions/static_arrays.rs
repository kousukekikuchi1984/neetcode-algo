use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        while let Some(index) = nums.iter().position(|v| *v == val) {
            nums.remove(index);
        }

        nums.len() as i32
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut cur = 0;
        for i in 1..nums.len() {
            if nums[cur] != nums[i] {
                cur += 1;
                nums[cur] = nums[i];
            }
        }
        return (cur + 1) as i32;
    }

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut cur_sum = 0;
        for n in nums {
            cur_sum = max(cur_sum, 0);
            cur_sum += n;
            max_sum = max(max_sum, cur_sum);
        }
        max_sum
    }

    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_remove_element() {
        let mut nums: Vec<i32> = vec![1];
        Solution::remove_element(&mut nums, 1);
    }

    #[test]
    fn test_max_sub_array() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn test_max_subarray_sum_circular() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
        assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
        assert_eq!(Solution::max_subarray_sum_circular(vec![-3, -2, -3]), -2);
    }
}
