use std::collections::HashMap;
use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for num in nums.iter() {
            if !set.insert(num) {
                return true;
            }
        }
        false
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // index -> complement
        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(val) = map.get(&complement) {
                return vec![*val, i as i32];
            }
            map.insert(num, i as i32);
        }
        vec![]
    }
}

struct LRUCache {
    capacity: i32,
    map: HashMap<i32, i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {}

    fn get(&self, key: i32) -> i32 {}

    fn put(&self, key: i32, value: i32) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
