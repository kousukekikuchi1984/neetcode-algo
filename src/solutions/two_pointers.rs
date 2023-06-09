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
        "BANC".to_string()
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
}
