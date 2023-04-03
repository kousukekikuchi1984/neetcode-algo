use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn _num_islands(
            grid: &Vec<Vec<char>>,
            current: (isize, isize),
            arrived: &mut HashSet<(isize, isize)>,
        ) {
            let (x, y) = current;
            // end recursive when: out of bounds, already arrived,
            if x < 0
                || y < 0
                || x >= grid.len() as isize
                || y >= grid[0].len() as isize
                || arrived.contains(&current)
            {
                return;
            }

            // end recursive when: current is water
            if grid[x as usize][y as usize] == '0' {
                return;
            }

            // update arrived
            arrived.insert(current);

            _num_islands(grid, (x + 1, y), arrived);
            _num_islands(grid, (x - 1, y), arrived);
            _num_islands(grid, (x, y + 1), arrived);
            _num_islands(grid, (x, y - 1), arrived);
        }

        let mut arrived: HashSet<(isize, isize)> = HashSet::new();
        let mut islands = 0;
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if grid[x][y] == '1' && !arrived.contains(&(x as isize, y as isize)) {
                    _num_islands(&grid, (x as isize, y as isize), &mut arrived);
                    islands += 1;
                }
            }
        }
        islands
    }

    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        fn _max_area_of_island(
            grid: &Vec<Vec<i32>>,
            current: (isize, isize),
            arrived: &mut HashSet<(isize, isize)>,
        ) -> i32 {
            let mut area = 1;
            let (x, y) = current;
            arrived.insert(current);
            for (x1, y1) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                let (x1, y1) = (*x1, *y1);
                let next = (x + x1, y + y1);
                if next.0 < 0
                    || next.1 < 0
                    || next.0 >= grid.len() as isize
                    || next.1 >= grid[0].len() as isize
                {
                    continue;
                }
                if grid[next.0 as usize][next.1 as usize] == 0 || arrived.contains(&next) {
                    continue;
                }
                area += _max_area_of_island(grid, next, arrived);
            }
            area
        }

        let mut arrived: HashSet<(isize, isize)> = HashSet::new();
        let mut max_area = 0;
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if grid[x][y] == 1 && !arrived.contains(&(x as isize, y as isize)) {
                    let area = _max_area_of_island(&grid, (x as isize, y as isize), &mut arrived);
                    if area > max_area {
                        max_area = area;
                    }
                }
            }
        }
        max_area
    }
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut queue = VecDeque::new();
        queue.push_back((0, 0, 1));
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        let directions = vec![
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];
        while let Some((r, c, length)) = queue.pop_front() {
            if r < 0 || r >= n || c < 0 || c >= n || grid[r][c] == 1 {
                continue;
            }
            if r == n - 1 && c == n - 1 {
                return length;
            }
            for (dr, dc) in directions.iter() {
                let (new_r, new_c) = (r as isize + dr, c as isize + dc);
                if visited.contains(&(new_r as usize, new_c as usize)) {
                    continue;
                }
                queue.push_back((new_r as usize, new_c as usize, length + 1));
                visited.insert((new_r as usize, new_c as usize));
            }
        }
        -1
    }

    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut queue: VecDeque<(isize, isize, i32)> = VecDeque::new();
        let mut arrived: HashSet<(usize, usize)> = HashSet::new();
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut fresh = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    fresh += 1;
                }
                if grid[i][j] == 2 {
                    for direction in directions.iter() {
                        let (dx, dy) = direction;
                        queue.push_back((i as isize + dx, j as isize + dy, 1));
                    }
                }
            }
        }
        let mut max_length = 0;
        while let Some(q) = queue.pop_front() {
            let (x, y, length) = q;
            // ignore when out of bounds
            if x < 0 || x >= grid.len() as isize || y < 0 || y >= grid[0].len() as isize {
                continue;
            }
            // empty cell
            if grid[x as usize][y as usize] == 0 {
                continue;
            }
            // already visited
            if arrived.contains(&(x as usize, y as usize)) {
                continue;
            }
            // rotten
            if grid[x as usize][y as usize] == 2 {
                continue;
            }
            // fresh
            if grid[x as usize][y as usize] == 1 {
                arrived.insert((x as usize, y as usize));
                fresh -= 1;
                for (dx, dy) in directions.iter() {
                    let (dx, dy) = (*dx, *dy);
                    queue.push_back((x + dx, y + dy, length + 1));
                }
            }
            if length > max_length {
                max_length = length;
            }
        }
        return if fresh == 0 { max_length } else { -1 };
    }

    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        fn _can_finish(
            graph: &HashMap<i32, Vec<i32>>,
            course: i32,
            visited: &mut HashSet<i32>,
            visiting: &mut HashSet<i32>,
        ) -> bool {
            if visiting.contains(&course) {
                return false;
            }
            if visited.contains(&course) {
                return true;
            }

            visiting.insert(course);
            visited.insert(course);
            if let Some(v) = graph.get(&course) {
                for pre in v.iter() {
                    if !_can_finish(graph, *pre, visited, visiting) {
                        return false;
                    }
                }
            }
            visiting.remove(&course)
        }

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        // build graph from prerequisites first.
        for prerequisite in prerequisites.iter() {
            let (course, pre) = (prerequisite[0], prerequisite[1]);
            if let Some(v) = graph.get_mut(&course) {
                v.push(pre);
            } else {
                graph.insert(course, vec![pre]);
            }
        }

        // visited -> already visited. if exists, return false
        let mut visited: HashSet<i32> = HashSet::new();

        // visiting is used for detecting cycle. if exists, return true
        let mut visiting: HashSet<i32> = HashSet::new();
        for course in 0..num_courses {
            if !visited.contains(&course) {
                if !_can_finish(&graph, course, &mut visited, &mut visiting) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_islands() {
        let grid: Vec<Vec<char>> = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let actual = Solution::num_islands(grid);
        let expected = 3;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_max_area_of_island() {
        let grid: Vec<Vec<i32>> = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        let actual = Solution::max_area_of_island(grid);
        let expected = 6;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_shortest_path_binary_matrix() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];
        let actual = Solution::shortest_path_binary_matrix(grid);
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_oranges_rotting() {
        let grid: Vec<Vec<i32>> = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        let actual = Solution::oranges_rotting(grid);
        let expected = 4;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_can_finish() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let actual = Solution::can_finish(num_courses, prerequisites);
        let expected = true;
        assert_eq!(actual, expected);
    }
}
