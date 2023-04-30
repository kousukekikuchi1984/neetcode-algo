use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

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
        fn dfs(
            email: &str,
            graph: &HashMap<String, HashSet<String>>,
            email_to_name: &HashMap<String, String>,
            visited: &mut HashSet<String>,
            merged: &mut Vec<String>,
        ) {
            visited.insert(email.to_owned());
            merged.push(email.to_owned());
            if let Some(neighbors) = graph.get(email) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        dfs(neighbor, graph, email_to_name, visited, merged);
                    }
                }
            }
        }

        let mut email_to_name: HashMap<String, String> = HashMap::new();
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

        // Build the graph
        for account in accounts.iter() {
            let name = &account[0];
            let emails = &account[1..];
            for i in 0..emails.len() {
                let email = &emails[i];
                email_to_name.insert(email.clone(), name.clone());
                let prev_email = &emails[0];
                if i > 0 {
                    graph
                        .entry(prev_email.clone())
                        .or_default()
                        .insert(email.clone());
                    graph
                        .entry(email.clone())
                        .or_default()
                        .insert(prev_email.clone());
                }
            }
        }

        // DFS the graph to collect merged accounts
        let mut merged_accounts: Vec<Vec<String>> = vec![];
        let mut visited: HashSet<String> = HashSet::new();
        for email in email_to_name.keys() {
            if !visited.contains(email) {
                let mut merged = vec![];
                dfs(email, &graph, &email_to_name, &mut visited, &mut merged);
                merged.sort();
                merged.insert(0, email_to_name.get(email).unwrap().to_owned());
                merged_accounts.push(merged);
            }
        }

        merged_accounts
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
        let accounts = vec![
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

        let expected = vec![
            vec![
                String::from("John"),
                String::from("john00@mail.com"),
                String::from("john_newyork@mail.com"),
                String::from("johnsmith@mail.com"),
            ],
            vec![String::from("John"), String::from("johnnybravo@mail.com")],
            vec![String::from("Mary"), String::from("mary@mail.com")],
        ];
        let actual = Solution::accounts_merge(accounts);
        for a in actual.iter() {
            assert!(expected.contains(a));
        }
    }
}
