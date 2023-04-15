use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet};

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
        // strategy: sum of subarray of size k with sliding window
        let mut sum = 0;
        let mut count = 0;
        for i in 0..arr.len() {
            sum += arr[i];
            if i >= k as usize {
                sum -= arr[i - k as usize];
            }
            if sum >= k * threshold && i >= k as usize - 1 {
                count += 1;
            }
        }
        count
    }

    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        // sliding window + two pointers
        let mut min_len = nums.len() + 1;
        let mut sum = 0;
        let mut left: usize = 0;
        for right in 0..nums.len() {
            sum += nums[right];
            while sum >= target {
                min_len = min(min_len, right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }
        if min_len == nums.len() + 1 {
            return 0;
        }
        min_len as i32
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut cur = 0;
        let mut left: usize = 0;
        let mut current_chars = HashSet::new();
        let chars: Vec<char> = s.chars().collect();
        for right in 0..s.len() {
            let current_char = chars[right];
            if current_chars.contains(&current_char) {
                while chars[left] != current_char {
                    current_chars.remove(&chars[left]);
                    left += 1;
                    cur -= 1;
                }
                left += 1;
                cur -= 1;
            }
            current_chars.insert(current_char);
            cur += 1;
            max_len = max(max_len, cur);
        }
        max_len
    }

    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut max_len = 0;
        let mut cur = 0;
        let mut left: usize = 0;
        let mut char_counts = HashMap::new();
        let chars = s.chars().collect::<Vec<char>>();
        for right in 0..chars.len() {
            *char_counts.entry(chars[right]).or_insert(0) += 1;
            cur = max(cur, char_counts[&chars[right]]);
            while right - left + 1 - cur > k as usize {
                *char_counts.get_mut(&chars[left]).unwrap() -= 1;
                left += 1;
            }
            max_len = max(max_len, right - left + 1);
        }
        max_len as i32
    }

    pub fn is_palindrome(s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();
        let mut left: usize = 0;
        let mut right = chars.len() - 1;
        while left <= right {
            if !chars[left].is_alphanumeric() {
                left += 1;
                continue;
            }
            if !chars[right].is_alphanumeric() {
                right -= 1;
                continue;
            }
            if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase() {
                return false;
            }
            if left == right {
                return true;
            }
            left += 1;
            right -= 1;
        }
        true
    }

    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        // two pointers
        let mut left: usize = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = numbers[left] + numbers[right];
            match target.cmp(&sum) {
                Ordering::Less => {
                    right -= 1;
                }
                Ordering::Equal => return vec![left as i32 + 1, right as i32 + 1],
                Ordering::Greater => {
                    left += 1;
                }
            }
        }
        unreachable!("No two sum solution");
    }

    pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = 1;
        let mut count = 1;
        while right < nums.len() {
            match nums[left].cmp(&nums[right]) {
                Ordering::Less => {
                    left = right;
                    count = 1;
                    right += 1;
                },
                Ordering::Equal => {
                    count += 1;
                    if count > 2 {
                        nums.remove(right);
                        continue;
                    }
                    right += 1;
                }
                Ordering::Greater => unreachable!("left should be fixed"),
            };
        }
        nums.len() as i32
    }

    pub fn max_area(height: Vec<i32>) -> i32 {

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
        assert_eq!(
            Solution::num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4),
            3
        );
    }

    #[test]
    fn test_min_sub_array_len() {
        // ref: https://leetcode.com/problems/minimum-size-subarray-sum/
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }

    #[test]
    fn test_length_of_longest_substring() {
        // ref: https://leetcode.com/problems/longest-substring-without-repeating-characters/
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    }

    #[test]
    fn test_character_replacement() {
        // ref: https://leetcode.com/problems/longest-repeating-character-replacement/
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
    }

    #[test]
    fn test_is_palindrome() {
        // https://leetcode.com/problems/valid-palindrome/
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
        assert_eq!(Solution::is_palindrome("a.".to_string()), true);
    }

    #[test]
    fn test_two_sum() {
        // https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn test_remove_duplicates() {
        // https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates_2(&mut nums), 5);
        assert_eq!(nums, vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_max_area() {
        // https://leetcode.com/problems/container-with-most-water/
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
