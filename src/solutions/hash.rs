use std::collections::HashSet;
use std::collections::{HashMap, VecDeque};

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
    queue: VecDeque<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            queue: VecDeque::with_capacity(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(val) => {
                self.queue.retain(|&x| x != key);
                self.queue.push_front(key);
                *val
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.queue.retain(|&x| x != key);
        } else if self.queue.len() == self.capacity as usize {
            let last = self.queue.pop_back().unwrap();
            self.map.remove(&last);
        }
        self.queue.push_front(key);
        self.map.insert(key, value);
    }
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

    #[test]
    fn test_lru_cache() {
        let mut cache = LRUCache::new(2);
        assert_eq!(cache.get(2), -1);
        cache.put(2, 6);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 6);
    }
}
