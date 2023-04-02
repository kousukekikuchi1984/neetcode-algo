use std::collections::{BinaryHeap, HashSet};

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
        let grid: Vec<Vec<i32>> = vec![
            vec![0, 1],
            vec![1, 0],
        ];
        let actual = Solution::shortest_path_binary_matrix(grid);
        let expected = 2;
        assert_eq!(actual, expected);
    }
}
