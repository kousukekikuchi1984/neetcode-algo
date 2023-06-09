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
        fn contains_submap(small: &HashMap<char, usize>, large: &HashMap<char, usize>) -> bool {
            for (key, value) in small.iter() {
                if !large.contains_key(key) || *large.get(key).unwrap() < *value {
                    return false;
                }
            }
            true
        }

        let mut target: HashMap<char, usize> = HashMap::with_capacity(t.len());
        for c in t.chars() {
            *target.entry(c).or_insert(0) += 1;
        }
        let mut result = "".to_string();
        let mut left = 0;
        let mut right = 0;
        let english_letters = 52;
        let mut seek: HashMap<char, usize> = HashMap::with_capacity(english_letters);

        let s_chars: Vec<char> = s.chars().collect(); // StringをVec<char>に変換

        while right < s_chars.len() {
            println!("right: {}, left: {}", right, left);
            let c = s_chars[right];
            *seek.entry(c).or_insert(0) += 1;

            if contains_submap(&target, &seek) {
                let candidate = s_chars[left..=right].iter().collect::<String>();
                if result.is_empty() || candidate.len() < result.len() {
                    result = candidate;
                }
                left += 1;
                while left <= right {
                    let pop = s_chars[left];
                    let count = seek.entry(pop).or_insert(0);
                    *count -= 1;

                    if contains_submap(&target, &seek) {
                        let candidate = s_chars[left..=right].iter().collect::<String>();
                        if candidate.len() < result.len() {
                            result = candidate;
                        }
                        left += 1;
                        continue;
                    }

                    if target.contains_key(&c) {
                        break;
                    } else {
                        left += 1;
                    }
                }
            }
            right += 1;
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
}
