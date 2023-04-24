use std::collections::HashMap;

struct Trie {
    children: HashMap<char, Trie>,
    word: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            word: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.children.entry(c).or_insert(Trie::new());
        }
        cur.word = true;
    }

    fn search(&mut self, word: String) -> bool {
        let mut cur = self;
        for c in word.chars() {
            cur = match cur.children.get_mut(&c) {
                Some(t) => t,
                None => return false,
            };
        }
        cur.word
    }

    fn starts_with(&mut self, prefix: String) -> bool {
        let mut cur = self;
        for c in prefix.chars() {
            cur = match cur.children.get_mut(&c) {
                Some(t) => t,
                None => return false,
            };
        }
        true
    }
}


#[cfg(test)]
mod test {
    use super::Trie;

    #[test]
    fn test_prefix_tree() {
        // ref: https://leetcode.com/problems/implement-trie-prefix-tree/
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
    }
}
