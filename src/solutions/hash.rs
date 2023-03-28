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

struct MyHashSet {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {}

    fn add(&self, key: i32) {}

    fn remove(&self, key: i32) {}

    fn contains(&self, key: i32) -> bool {}
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

    #[test]
    fn test_my_hash_set() {
        let hash_set = MyHashSet::new();
        hash_set.add(1);
        hash_set.add(2);
        assert_eq!(hash_set.contains(1), true);
        assert_eq!(hash_set.contains(3), false);
        hash_set.add(2);
        assert_eq!(hash_set.contains(2), true);
        hash_set.remove(2);
        assert_eq!(hash_set.contains(2), false);
    }
}
