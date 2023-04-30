use std::cmp::Ordering;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..(n + 1)).collect(),
            rank: vec![1; n + 1],
        }
    }

    fn find(&mut self, n: usize) -> usize {
        let mut p = self.parent[n];

        while p != self.parent[p] {
            self.parent[p] = self.parent[self.parent[p]];
            p = self.parent[p];
        }

        p
    }

    fn union(&mut self, n1: usize, n2: usize) -> bool {
        let p1 = self.find(n1);
        let p2 = self.find(n2);

        if p1 == p2 {
            return false;
        }
        match self.rank[p1].cmp(&self.rank[p2]) {
            Ordering::Greater => {
                self.parent[p2] = p1;
                self.rank[p1] += self.rank[p2];
            }
            _ => {
                self.parent[p1] = p2;
                self.rank[p2] = self.rank[p1];
            }
        }
        true
    }
}

struct Solution {}
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut union_find = UnionFind::new(edges.len() + 1);

        for edge in edges {
            let (n1, n2) = (edge[0] as usize, edge[1] as usize);
            if !union_find.union(n1, n2) {
                return vec![n1 as i32, n2 as i32];
            }
        }
        unreachable!()
    }

    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_redundant_connection() {
        let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        let expected = vec![2, 3];
        assert_eq!(Solution::find_redundant_connection(edges), expected);
    }

    #[test]
    fn test_accounts_merge() {
        // ref: https://leetcode.com/problems/accounts-merge/
        let mut accounts = vec![
            vec![
                "John".to_string(),
                "johnsmith@mail.com".to_string(),
                "john_newyork@mail.com".to_string(),
            ],
            vec![
                "John".to_string(),
                "johnsmith@mail.com".to_string(),
                "john00@mail.com".to_string(),
            ],
            vec!["Mary".to_string(), "mary@mail.com".to_string()],
            vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
        ];
        let expected = [
            [
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com",
            ],
            ["Mary", "mary@mail.com"],
            ["John", "johnnybravo@mail.com"],
        ]
        .into_iter()
        .map(|v| {
            vec![v[0].to_string()]
                .into_iter()
                .chain(v[1..].into_iter().map(|s| s.to_string()))
                .collect()
        })
        .collect();
        assert_eq!(Solution::accounts_merge(accounts), expected);
    }
}
