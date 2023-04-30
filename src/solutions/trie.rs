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
            }
            _ => match self.children.get(&c) {
                Some(child) => child._search(&word[1..]),
                None => false,
            },
        }
    }
}

struct Solution {}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        pub fn dfs(
            board: &mut Vec<Vec<char>>,
            mut trie: &mut TrieFW,
            res: &mut Vec<String>,
            word: &mut String,
            i: usize,
            j: usize,
        ) {
            let (m, n) = (board.len(), board[0].len());

            if i != usize::MAX && j != usize::MAX && i < m && j < n && board[i][j] != '#' {
                let c = board[i][j];

                // mark as visited
                board[i][j] = '#';

                if let Some(ref mut curr_trie) = trie.children[char_index(c)] {
                    trie = curr_trie.as_mut();

                    word.push(c);

                    if trie.is_word_end {
                        res.push(word.clone());
                        trie.is_word_end = false;
                    }

                    dfs(board, trie, res, word, i + 1, j);
                    dfs(board, trie, res, word, i, j + 1);
                    if i > 0 {
                        dfs(board, trie, res, word, i - 1, j);
                    }
                    if j > 0 {
                        dfs(board, trie, res, word, i, j - 1);
                    }

                    word.pop();
                }

                board[i][j] = c;
            }
        }

        // return word
        let mut trie = TrieFW::new();
        let (m, n) = (board.len(), board[0].len());

        for word in words{
            trie.insert(word);
        }

        let mut word = String::new();
        let mut res = vec![];

        for i in 0..m{
            for j in 0..n{
                dfs(&mut board, &mut trie, &mut res, &mut word,  i, j);
            }
        }
        res
    }
}

#[derive(Default, Debug, Clone)]
struct TrieFW {
    is_word_end: bool,
    children: [Option<Box<TrieFW>>; 26],
}

impl TrieFW {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut trie = self;

        for c in word.chars() {
            trie = trie.children[char_index(c)]
                .get_or_insert(Box::new(TrieFW::new()))
                .as_mut();
        }

        trie.is_word_end = true;
    }
}

fn char_index(c: char) -> usize {
    (c as u8 - 'a' as u8) as usize
}

// struct WordFilter {
//     is_word_end: bool,
//     children: [Option<Box<WordFilter>>; 26],
// }
//
// impl WordFilter {
//
//     fn new(words: Vec<String>) -> Self {
//         let mut root = WordFilter{
//             is_word_end: false,
//             children: [None; 26],
//         };
//         for word in words {
//             let mut trie = &mut root;
//             for c in word.chars() {
//                 trie = trie.children[char_index(c)]
//                     .get_or_insert_with(|| Box::new(WordFilter{
//                         is_word_end: false,
//                         children: [None; 26],
//                     }))
//                     .as_mut();
//             }
//             trie.is_word_end = true;
//         }
//         root
//     }
//
//     fn f(&self, pref: String, suff: String) -> i32 {
//
//     }
// }

#[cfg(test)]
mod test {
    use super::Solution;
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

    #[test]
    fn test_find_words() {
        // ref: https://leetcode.com/problems/word-search-ii/
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = vec!["oath", "pea", "eat", "rain"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = vec!["oath", "eat"];
        assert_eq!(Solution::find_words(board, words), expected);
    }

    // #[test]
    // fn test_prefix_and_suffix_search() {
    //     // ref: https://leetcode.com/problems/prefix-and-suffix-search/
    //     let words = vec!["apple", "apply", "app", "ap", "a"]
    //         .iter()
    //         .map(|s| s.to_string())
    //         .collect();
    //     let word_filter = WordFilter::new(words);
    //     assert_eq!(word_filter.f("a".to_string(), "e".to_string()), 1);
    //     assert_eq!(word_filter.f("a".to_string(), "p".to_string()), 2);
    //     assert_eq!(word_filter.f("a".to_string(), "pp".to_string()), 2);
    //     assert_eq!(word_filter.f("ap".to_string(), "e".to_string()), 2);
    //     assert_eq!(word_filter.f("ap".to_string(), "p".to_string()), 2);
    //     assert_eq!(word_filter.f("ap".to_string(), "pp".to_string()), 2);
    //     assert_eq!(word_filter.f("app".to_string(), "e".to_string()), 2);
    //     assert_eq!(word_filter.f("app".to_string(), "p".to_string()), 2);
    //     assert_eq!(word_filter.f("app".to_string(), "pp".to_string()), 2);
    //     assert_eq!(word_filter.f("appl".to_string(), "e".to_string()), 1);
    //     assert_eq!(word_filter.f("appl".to_string(), "p".to_string()), 1);
    //     assert_eq!(word_filter.f("appl".to_string(), "pp".to_string()), 1);
    //     assert_eq!(word_filter.f("apple".to_string(), "e".to_string()), 0);
    //     assert_eq!(word_filter.f("apple".to_string(), "p".to_string()), 0);
    //     assert_eq!(word_filter.f("apple".to_string(), "pp".to_string()), 0);
    //     assert_eq!(word_filter.f("apply".to_string(), "e".to_string()), 0);
    //     assert_eq!(word_filter.f("apply".to_string(), "p".to_string()), 1);
    //     assert_eq!(word_filter.f("apply".to_string(), "pp".to_string()), 1);
    // }
}
