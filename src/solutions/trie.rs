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

struct WordDictionary {
    children: HashMap<char, WordDictionary>,
    word: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            word: false,
        }
    }

    fn add_word(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.children.entry(c).or_insert(WordDictionary::new());
        }
        cur.word = true;
    }

    fn search(&mut self, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        self._search(&word)
    }

    fn _search(&self, word: &[char]) -> bool {
        if word.is_empty() {
            return self.word;
        }

        let c = word[0];
        match c {
            '.' => {
                for (_, child) in self.children.iter() {
                    if child._search(&word[1..]) {
                        return true;
                    }
                }
                false
            },
            _ => {
                match self.children.get(&c) {
                    Some(child) => child._search(&word[1..]),
                    None => false,
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::Trie;
    use super::WordDictionary;

    #[test]
    fn test_prefix_tree() {
        // ref: https://leetcode.com/problems/implement-trie-prefix-tree/
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
    }

    #[test]
    fn test_word_dictionary() {
        // ref: https://leetcode.com/problems/design-add-and-search-words-data-structure/
        let mut wd = WordDictionary::new();
        wd.add_word("bad".to_string());
        wd.add_word("dad".to_string());
        wd.add_word("mad".to_string());
        wd.add_word("pad".to_string());
        assert_eq!(wd.search("bad".to_string()), true);
        assert_eq!(wd.search("dad".to_string()), true);
        assert_eq!(wd.search("mad".to_string()), true);
        assert_eq!(wd.search("pad".to_string()), true);
        assert_eq!(wd.search(".ad".to_string()), true);
        assert_eq!(wd.search("b..".to_string()), true);
        assert_eq!(wd.search("b...".to_string()), false);
    }
}
