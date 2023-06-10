use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 3 <= nums.length <= 3000
        // n^2 and hashmap
        let mut three = HashMap::with_capacity(nums.len());
        for i in 0..nums.len() {
            three.insert(nums[i], i);
        }
        let mut result: HashSet<Vec<i32>> = HashSet::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let expected_value = (nums[i] + nums[j]) * -1;
                if three.contains_key(&expected_value) {
                    let index = three.get(&expected_value).unwrap();
                    if *index > j {
                        let mut triplet = vec![nums[i], nums[j], expected_value];
                        triplet.sort();
                        result.insert(triplet);
                    }
                }
            }
        }
        result.into_iter().collect()
    }

    pub fn min_window(s: String, t: String) -> String {
        let mut target_counts: HashMap<char, usize> = HashMap::new();
        for c in t.chars() {
            *target_counts.entry(c).or_insert(0) += 1;
        }

        let s_chars: Vec<char> = s.chars().collect();
        let mut window_counts: HashMap<char, usize> = HashMap::new();

        let mut left = 0;
        let mut right = 0;
        let mut min_window_size = s.len() + 1;
        let mut min_window_start = 0;

        let mut matched_chars = 0;

        while right < s_chars.len() {
            let c = s_chars[right];
            *window_counts.entry(c).or_insert(0) += 1;

            if let Some(&target_count) = target_counts.get(&c) {
                if window_counts[&c] <= target_count {
                    matched_chars += 1;
                }
            }

            while matched_chars == t.len() {
                let window_size = right - left + 1;
                if window_size < min_window_size {
                    min_window_size = window_size;
                    min_window_start = left;
                }

                let left_char = s_chars[left];
                *window_counts.get_mut(&left_char).unwrap() -= 1;

                if let Some(&target_count) = target_counts.get(&left_char) {
                    if window_counts[&left_char] < target_count {
                        matched_chars -= 1;
                    }
                }

                left += 1;
            }

            right += 1;
        }

        if min_window_size > s.len() {
            "".to_string()
        } else {
            s_chars[min_window_start..min_window_start + min_window_size]
                .iter()
                .collect()
        }
    }

    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = vec![];
        let mut subarray = nums[0..k].to_vec();
        let mut current = *subarray.iter().max().unwrap();
        result.push(current);
        let mut cur = 1;

        while cur + k - 1 < nums.len() {
            let val = nums[cur + k - 1];
            let prev = subarray.remove(0);
            if current == prev {
                subarray.push(val);
                current = *subarray.iter().max().unwrap();
                result.push(current);
            } else {
                current = current.max(val);
                result.push(current);
                subarray.push(val);
            }
            cur += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expected_vec = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        let expected: HashSet<Vec<i32>> = HashSet::from_iter(expected_vec);
        let act_vec = Solution::three_sum(nums);
        let actual: HashSet<Vec<i32>> = HashSet::from_iter(act_vec);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_min_window() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let actual = Solution::min_window(s, t);
        let expected = "BANC".to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_max_sliding_window() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let actual = Solution::max_sliding_window(nums, k);
        let expected = vec![3, 3, 5, 5, 6, 7];
        assert_eq!(actual, expected);
    }
}
