use std::cmp::{max, min, Ordering};
use std::collections::HashSet;

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

    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut global_max, mut global_min) = (nums[0], nums[0]);
        let (mut current_max, mut current_min) = (0, 0);
        let mut total = 0;

        for num in nums {
            current_max = i32::max(num, current_max + num);
            current_min = i32::min(num, current_min + num);
            total += num;
            global_max = i32::max(global_max, current_max);
            global_min = i32::min(global_min, current_min);
        }

        return if global_max > 0 {
            i32::max(global_max, total - global_min)
        } else {
            global_max
        };
    }

    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.len() < 2 {
            return arr.len() as i32;
        }
        let mut max_len = 1;
        let mut cur_len = 1;
        let mut prev_cmp = None;
        for i in 1..arr.len() {
            let cmp = arr[i - 1].cmp(&arr[i]);
            if cmp == Ordering::Equal {
                cur_len = 1;
                prev_cmp = None;
            } else if let Some(prev) = prev_cmp {
                if prev != cmp {
                    cur_len += 1;
                    max_len = max(max_len, cur_len);
                } else {
                    cur_len = 2;
                }
                prev_cmp = Some(cmp);
            } else {
                cur_len += 1;
                max_len = max(max_len, cur_len);
                prev_cmp = Some(cmp);
            }
        }
        max_len
    }

    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut window = HashSet::new();
        let mut left: usize = 0;
        for right in 0..nums.len() {
            if right - left > k as usize {
                let val = nums[left];
                window.remove(&val);
                left += 1;
            }
            if window.contains(&nums[right]) {
                return true;
            }
            window.insert(nums[right]);
        }
        false
    }

    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {

    }
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

    #[test]
    fn test_max_turbulence_size() {
        assert_eq!(
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]),
            5
        );
        assert_eq!(Solution::max_turbulence_size(vec![4, 8, 12, 16]), 2);
        assert_eq!(
            Solution::max_turbulence_size(vec![0, 1, 1, 0, 1, 0, 1, 1, 0, 0]),
            5
        );
    }

    #[test]
    fn test_contains_nearby_duplicate() {
        // ref: https://leetcode.com/problems/contains-duplicate-ii/
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
            true
        );
    }

    #[test]
    fn test_num_of_subarrays() {
        // ref: https://leetcode.com/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/
        assert_eq!(Solution::num_of_subarrays(vec![2,2,2,2,5,5,5,8], 3, 4), 3);
    }
}
