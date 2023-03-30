use std::collections::HashSet;
use std::collections::LinkedList;
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

struct MyHashSet {
    data: Vec<Vec<i32>>, // 997 because 10^6 ~= 997^2
}

impl MyHashSet {
    fn new() -> Self {
        let mut table: Vec<Vec<i32>> = vec![];
        for _ in 0..997 {
            let row: Vec<i32> = vec![];
            table.push(row);
        }
        MyHashSet { data: table }
    }

    fn add(&mut self, key: i32) {
        let h = self.hash(key) as usize;
        for val in &self.data[h] {
            if val == &key {
                return;
            }
        }
        self.data[h].push(key);
    }

    fn remove(&mut self, key: i32) {
        let h = self.hash(key) as usize;
        let i = self.data[h].iter().position(|&x| x == key);
        if i != None {
            self.data[h].remove(i.unwrap());
        }
    }

    fn contains(&self, key: i32) -> bool {
        let h = self.hash(key) as usize;
        let i = self.data[h].iter().position(|&x| x == key);
        i != None
    }

    // calculate the entry
    fn hash(&self, key: i32) -> i32 {
        key % 997
    }
}

const BUCKET_COUNT: usize = 100;

#[derive(Default)]
struct MyHashMap {
    buckets: Vec<LinkedList<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            buckets: vec![LinkedList::new(); BUCKET_COUNT],
        }
    }

    fn key_hash(key: i32) -> usize {
        (key as usize) % BUCKET_COUNT
    }

    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        let hash = Self::key_hash(key);
        for node in self.buckets[hash].iter_mut() {
            if node.0 == key {
                node.1 = value;
                return;
            }
        }
        self.buckets[hash].push_back((key, value));
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {
        let hash = Self::key_hash(key);
        for node in self.buckets[hash].iter() {
            if node.0 == key {
                return node.1;
            }
        }
        -1
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        let hash = Self::key_hash(key);
        let idx = self.buckets[hash]
            .iter()
            .enumerate()
            .find_map(|(idx, node)| if node.0 == key { Some(idx) } else { None });
        if let Some(idx) = idx {
            let mut split_list = self.buckets[hash].split_off(idx);
            split_list.pop_front();
            self.buckets[hash].append(&mut split_list);
        }
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

    #[test]
    fn test_my_hash_set() {
        let mut hash_set = MyHashSet::new();
        hash_set.add(1);
        hash_set.add(2);
        assert_eq!(hash_set.contains(1), true);
        assert_eq!(hash_set.contains(3), false);
        hash_set.add(2);
        assert_eq!(hash_set.contains(2), true);
        hash_set.remove(2);
        assert_eq!(hash_set.contains(2), false);
    }

    #[test]
    fn test_my_hash_map() {
        let mut hash_map = MyHashMap::new();
        hash_map.put(1, 1);
        hash_map.put(2, 2);
        assert_eq!(hash_map.get(1), 1);
        assert_eq!(hash_map.get(3), -1);
        hash_map.put(2, 1);
        assert_eq!(hash_map.get(2), 1);
        hash_map.remove(2);
        assert_eq!(hash_map.get(2), -1);
    }
}
